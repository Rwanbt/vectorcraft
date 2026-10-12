# Test execution and exact-SHA evidence

**Repo:** `Rwanbt/vectorcraft`
**Base:** `main@436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Branch:** `test/fork-quality-baseline-20261012`
**Last fully verified head:** `0bc251fad1aa442d8d56e0d1725a9beeb4d22e59`
**Current classification:** PARTIAL; a strict JSON-RPC protocol-version fix and new Windows/macOS document/MCP gates were pushed after the last fully green run.

## Verified checkpoint: 2026-10-12

[GitHub Actions run 38313490657](https://github.com/Rwanbt/vectorcraft/actions/runs/38313490657) at exact SHA `0bc251fad1aa442d8d56e0d1725a9beeb4d22e59` (**8/8 jobs passed**), plus Affinity and packaging workflows:

| Job | Result / archived evidence |
| --- | --- |
| Linux complete workspace | **PASS:** `cargo xtask ci` — Rust fmt, workspace Clippy -D warnings, all workspace tests, assets, brand check, dependency layers (27 crates), WASM check |
| Exact harness inventory | **PASS:** `cargo xtask stats --exact`, artifact `11777083159`; TOTAL row: 5,340 discovered tests, 40 ignored, final column 5,305 |
| Geometry Linux LLVM | **PASS:** 70 unit + 3 seeded metamorphic + 19 integration/property = 92/92. Isolated 96.36% lines, artifact `11775902884` |
| Geometry Windows / macOS | **PASS:** `cargo test -p vectorcraft-geom --locked` on both OS runners |
| Native format Linux LLVM | **PASS:** 48 tests across six harness results; isolated 97.00% lines, artifact `11776505239` |
| Engine Linux LLVM | **PASS:** isolated 96.73% lines, 94.42% functions, 289 source files, artifact `11776456712` |
| MCP Linux LLVM | **PASS:** isolated 96.13% lines, 90.88% functions, 20 source files, artifact `11776097634` |
| Fast rustfmt | **PASS** before expensive build jobs |
| Affinity corpus / Packaging lint | **PASS:** PR workflows on the identical head SHA |

Exact pass counts are based on each runner's output; the exact harness inventory is not the same measurement as the executed test result tally.

## Added after the checkpoint

- Strict validation of `jsonrpc: "2.0"` in the MCP server *before* processing requests or notifications; regression verifies invalid/missing versions never mutate the document or initialize the session.
- Portable macOS + Windows functional test matrix for `vectorcraft-format` and `vectorcraft-mcp` (in addition to geometry).
- These additions need a new **exact head SHA** CI result. Prior successes are historical, not evidence for the new commit.

## Not yet demonstrated

Whole-product GUI desktop/browser launch-edit-undo-save-restart-export E2E, install/uninstall lifecycle, all-platform full workspace test matrix, and complete branch-sensitive coverage. Do not certify production readiness or merge the draft PR until those risks are resolved or explicitly scoped.
