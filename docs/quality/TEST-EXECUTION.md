# Test execution and exact-SHA evidence

**Repo:** `Rwanbt/vectorcraft`
**Base:** `main@436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Branch:** `test/fork-quality-baseline-20261012`
**Last fully verified head:** `95270cf4ef5cbe8cacf8432029da60f86e17b40b`
**Current classification:** PARTIAL; loss-of-acknowledgement protection and three remote socket tests were pushed after the last fully green run and still require exact-SHA validation.

## Verified checkpoint: 2026-10-13

[GitHub Actions run 38320403378](https://github.com/Rwanbt/vectorcraft/actions/runs/38320403378) at exact SHA `95270cf4ef5cbe8cacf8432029da60f86e17b40b` (**10/10 jobs passed**), plus successful Affinity and packaging workflows on the same commit:

| Job | Result / archived evidence |
| --- | --- |
| Linux complete workspace | **PASS:** `cargo xtask ci` — Rust fmt, workspace Clippy -D warnings, all workspace tests, assets, brand check, dependency layers (27 crates), WASM check |
| Exact harness inventory | **PASS:** `cargo xtask stats --exact`, artifact `11780291784`; TOTAL row: 5,341 discovered tests, 40 ignored, final column 5,306 |
| Geometry Linux LLVM | **PASS:** 70 unit + 3 seeded metamorphic + 19 integration/property = 92/92. Isolated 96.36% lines, artifact `11779066391` |
| Geometry Windows / macOS | **PASS:** `cargo test -p vectorcraft-geom --locked` on both OS runners |
| Native format Linux LLVM | **PASS:** 48 tests across six harness results; isolated 97.00% lines, artifact `11778757504` |
| Engine Linux LLVM | **PASS:** isolated 96.73% lines, 94.41% functions, 289 source files, artifact `11779107961` |
| MCP Linux LLVM | **PASS:** isolated 96.14% lines, 90.91% functions, 20 source files, artifact `11779197141` |
| Native format + MCP Windows/macOS | **PASS:** `cargo test -p vectorcraft-format -p vectorcraft-mcp --locked` on both platforms |
| Fast rustfmt | **PASS** before expensive build jobs |
| Affinity corpus / Packaging lint | **PASS:** PR workflows on the identical head SHA |

Exact pass counts are based on each runner's output; the exact harness inventory is not the same measurement as the executed test result tally.

## Added after the verified checkpoint

- `Remote::call` used to transparently replay a command if a control TCP reply was lost. Because an app may have *executed* an action before dropping its reply, the retry could duplicate a deletion, insertion or export.
- New fail-closed behavior performs **at most one transport attempt per explicit command**, treats loss, malformed reply or an unexpected response ID as an unknown outcome, drops the socket, and allows the next *explicit* command to reconnect.
- `crates/mcp/tests/remote_delivery.rs` exercises the three failure scenarios over real loopback sockets; each scenario checks that the action was not replayed, the next request connects afresh, and request IDs remain distinct.
- The [latest quality workflow](https://github.com/Rwanbt/vectorcraft/actions/workflows/fork-quality.yml) must pass on the *new* commit before these tests can be marked verified.

## Not yet demonstrated

Whole-product GUI desktop/browser launch-edit-undo-save-restart-export E2E, install/uninstall lifecycle, all-platform full workspace test matrix, and complete branch-sensitive coverage. Do not certify production readiness or merge the draft PR until those risks are resolved or explicitly scoped.
