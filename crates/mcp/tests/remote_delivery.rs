//! Remote control channel delivery contracts against a real loopback TCP socket.
//! Dropped or malformed acknowledgements cannot cause a mutating action to be replayed.
//! Integration tests may unwrap and panic to report failures.
//!
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::Duration;

use serde_json::{Value, json};
use vectorcraft_mcp::{Backend, Remote};

/// The app accepts a command and then drops its acknowledgement, sends malformed JSON, or
/// replies with a different request ID. A subsequent *explicitly initiated* call must reconnect.
fn exercise_failed_ack(first_reply: Option<&str>, expected_error: &str) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let first_reply = first_reply.map(str::to_owned);
    let server = std::thread::spawn(move || {
        let mut seen = Vec::new();
        for attempt in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut request = String::new();
            {
                let mut reader = BufReader::new(&stream);
                assert_ne!(reader.read_line(&mut request).unwrap(), 0);
            }
            let parsed: Value = serde_json::from_str(&request).unwrap();
            if attempt == 0 {
                if let Some(reply) = &first_reply {
                    stream.write_all(reply.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            } else {
                let answer = json!({"id": parsed["id"], "ok": true, "result": {"alive": true}});
                writeln!(stream, "{answer}").unwrap();
            }
            seen.push(parsed);
        }
        seen
    });

    let mut remote = Remote::connect(&addr.to_string()).unwrap();
    let err = remote.call("engine.execute", json!({"command": "edit.clear", "params": {}})).unwrap_err();
    assert!(err.contains(expected_error), "{err}");
    let response = remote.call("document.inspect", json!({})).unwrap();
    assert_eq!(response, json!({"alive": true}));
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2, "the failed action must not be sent twice");
    assert_eq!(requests[0]["method"], "engine.execute");
    assert_eq!(requests[1]["method"], "document.inspect");
    assert_eq!(requests[0]["id"], 1);
    assert_eq!(requests[1]["id"], 2);
}

#[test]
fn lost_acknowledgement_never_replays_mutating_command() {
    exercise_failed_ack(None, "outcome unknown, not retried");
}

#[test]
fn malformed_reply_does_not_replay_mutating_command() {
    exercise_failed_ack(Some("{not-json\n"), "bad reply from app");
}

#[test]
fn reply_with_wrong_request_id_is_rejected_without_replay() {
    exercise_failed_ack(Some("{\"id\":999,\"ok\":true,\"result\":{}}\n"), "wrong reply id");
}
