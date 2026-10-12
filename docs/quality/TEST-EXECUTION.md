# Test execution and exact-SHA evidence

**Repo:** `Rwanbt/vectorcraft`
**Base:** `main@436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Branch:** `test/fork-quality-baseline-20261012`
**Last fully verified head:** `14dff8f966873f7ab65bcf9de6bdfee141b6ca2a`
**Current classification:** PARTIAL; additional MCP tests and engine/MCP instrumentation were pushed after the last fully green run.

## Verified checkpoint: 2026-10-12

[GitHub Actions run 38278037131](https://github.com/Rwanbt/vectorcraft/actions/runs/38278037131) at exact SHA `14dff8f966873f7ab65bcf9de6bdfee141b6ca2a`:

| Job | Result / archived evidence |
| --- | --- |
| Linux complete workspace | **PASS:** `cargo xtask ci` — Rust fmt, workspace Clippy -D warnings, all workspace tests, assets, brand check, dependency layers (27 crates), WASM check |
| Exact harness inventory | **PASS:** `cargo xtask stats --exact`, artifact `11763853322`; TOTAL row: 5,335 discovered tests, 40 ignored, final column 5,300 |
| Geometry Linux LLVM | **PASS:** 70 unit + 3 seeded metamorphic + 19 integration/property = 92/92. Isolated 96.36% lines, artifact `11759364528` |
| Geometry Windows / macOS | **PASS:** `cargo test -p vectorcraft-geom --locked` on both OS runners |
| Native format Linux LLVM | **PASS:** 48 tests across six harness results; isolated 97.00% lines, artifact `11760706753` |
| Affinity corpus / Packaging lint | **PASS:** PR workflows on the identical head SHA |

Exact pass counts are based on each runner's output; the exact harness inventory is not the same measurement as the executed test result tally.

## Added after the checkpoint

- Four MCP protocol tests: mixed JSON-RPC batches and notifications, unknown-tool-argument rejection without document mutation, modern/legacy cache policy and stdio request/reply framing.
- Fast `rustfmt` preflight to catch formatting before slow build jobs.
- Independent instrumented LLVM coverage for `vectorcraft-engine` and `vectorcraft-mcp` (package-scoped with cross-crate source rejection).
- The new [fork quality workflow](https://github.com/Rwanbt/vectorcraft/actions/workflows/fork-quality.yml) must pass at the **latest PR head**, not merely at the checkpoint above.

## Not yet demonstrated

Whole-product GUI desktop/browser launch-edit-undo-save-restart-export E2E, install/uninstall lifecycle, all-platform full workspace test matrix, and complete branch-sensitive coverage. Do not certify production readiness or merge the draft PR until those risks are resolved or explicitly scoped.
