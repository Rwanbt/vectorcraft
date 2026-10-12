# VectorCraft fork: test and coverage baseline

**Audit date:** 2026-10-12
**Repository:** `Rwanbt/vectorcraft`
**Base:** `main` at `436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Audit branch:** `test/fork-quality-baseline-20261012`
**Status:** PARTIALLY VERIFIED — no measured coverage at audit time.

## Evidence collected before the first fork quality run

- Repository tree at the base commit: 1,548 tracked files, including 1,100 Rust source files.
- Workspace: 23 crates in `crates/`, 3 application directories in `apps/`, and `xtask` (27 members/directories total; verify Cargo's exact resolved member count in CI).
- 73 `*.rs` files under crate/app integration-test directories. This is **not a test case count**; inline unit tests and `#[test]` / `proptest` cases inside each file are not counted here.
- Existing CI definitions: six workflows (Affinity corpus, FreeBSD, packaging lint, releases, Windows ARM64 and Windows 7). There was no general PR-wide `cargo xtask ci` workflow.
- GitHub Actions workflow runs on the fork before this branch: zero. No tests or coverage measurements can be reported as passing on the fork from that observation.
- Existing test code includes `crates/geom/tests/prop_geom.rs`, `crates/format/tests/prop_format.rs`, `crates/engine/tests/import_fuzz.rs` and `crates/mcp/tests/automation_roots.rs`. Their outcomes on this fork are initially **UNVERIFIED**.
- The upstream `storytold/vectorcraft` had 349 commits beyond the fork base at comparison time. No upstream commits were merged into this QA branch.

## Instrumentation added in this branch

The `Fork quality and coverage` workflow runs the canonical `cargo xtask ci` on Linux; it exercises the geometry crate on Windows and macOS; and it generates real LLVM LCOV + JSON artifacts for **the geometry crate only**. It triggers on PRs and `main`, never updates the upstream, and has read-only token permissions.

To reproduce the initial measured scope:

```sh
cargo test -p vectorcraft-geom --locked
cargo llvm-cov -p vectorcraft-geom --locked --lcov --output-path coverage/geom.lcov
cargo llvm-cov report --json --output-path coverage/geom.json
cargo llvm-cov report --summary-only
cargo xtask ci
```

**Initial geometry line/function/region coverage:** NOT MEASURED (awaiting real GitHub Actions artifacts).

**Full-workspace line/function/branch coverage:** NOT MEASURED (the first instrumented job deliberately scopes geometry; expanding to all crates is remaining work).

**Exact test runner count:** NOT COLLECTED. The workspace provides `cargo xtask stats --exact`; run it with a suitable build runner and archive its output before asserting an exact count.

Do not assign `VERIFIED` based on the presence of tests, nor assert a 100% target before instrumented evidence and justified exclusions.
