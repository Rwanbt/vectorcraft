# VectorCraft fork: test and coverage baseline

**Audit date:** 2026-10-12
**Repository:** `Rwanbt/vectorcraft`
**Base:** `main` at `436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Audit branch:** `test/fork-quality-baseline-20261012`
**Status:** PARTIALLY VERIFIED — all 10 quality jobs and the Affinity/Packaging workflows passed at `95270cf4ef5cbe8cacf8432029da60f86e17b40b`. Remote control acknowledgement/replay hardening was added after this verified checkpoint and awaits fresh CI evidence.

## Evidence collected before the first fork quality run

- Repository tree at the base commit: 1,548 tracked files, including 1,100 Rust source files.
- Workspace: 23 crates in `crates/`, 3 application directories in `apps/`, and `xtask` (27 members/directories total; verify Cargo's exact resolved member count in CI).
- 73 `*.rs` files under crate/app integration-test directories. This is **not a test case count**; inline unit tests and `#[test]` / `proptest` cases inside each file are not counted here.
- Existing CI definitions: six workflows (Affinity corpus, FreeBSD, packaging lint, releases, Windows ARM64 and Windows 7). There was no general PR-wide `cargo xtask ci` workflow.
- GitHub Actions workflow runs on the fork before this branch: zero. No tests or coverage measurements can be reported as passing on the fork from that observation.
- Existing test code includes `crates/geom/tests/prop_geom.rs`, `crates/format/tests/prop_format.rs`, `crates/engine/tests/import_fuzz.rs` and `crates/mcp/tests/automation_roots.rs`. Their outcomes on this fork are initially **UNVERIFIED**.
- The upstream `storytold/vectorcraft` had 349 commits beyond the fork base at comparison time. No upstream commits were merged into this QA branch.

## Instrumentation added in this branch

The `Fork quality and coverage` workflow runs the canonical `cargo xtask ci` on Linux, exercises geometry on Windows and macOS, and generates package-isolated LLVM LCOV + JSON artifacts for geometry, the native format and (on newer commits) engine/MCP. All workflow permissions are read-only. It triggers on PRs and `main`, never updates the upstream, and has read-only token permissions.

To reproduce the initial measured scope:

```sh
cargo test -p vectorcraft-geom --locked
cargo llvm-cov -p vectorcraft-geom --locked --no-report
cargo llvm-cov report -p vectorcraft-geom --lcov --output-path coverage/geom.lcov
cargo llvm-cov report -p vectorcraft-geom --json --output-path coverage/geom.json
cargo llvm-cov report -p vectorcraft-geom --summary-only
cargo xtask ci
```

**First actual geometry-only measured values (source-file rows, SHA `8f04b29f8c965b1b43362cebdef3d87b8d106d82`):** lines 1,925/2,034 = **94.64%**; functions 302/326 = **92.64%**; regions 4,857/5,158 = **94.16%**. Source: [LLVM coverage run 38269159711](https://github.com/Rwanbt/vectorcraft/actions/runs/38269159711), job `Linux - instrumented geometry baseline`. The initial unscoped `cargo llvm-cov report` command also included unrelated crates, resulting in a misleading workspace-wide TOTAL of 1.69% lines; **do not cite that number as geometry or whole-workspace coverage**. This PR changes report generation to `report -p vectorcraft-geom` and adds a JSON source-scope guard. Remeasure after the additional tests; earlier numbers are a baseline, not the final PR value.

**Fully green CI at SHA `95270cf4ef5cbe8cacf8432029da60f86e17b40b`:** [run 38320403378](https://github.com/Rwanbt/vectorcraft/actions/runs/38320403378), **all ten jobs passed**, including Windows/macOS format and MCP portability, plus Affinity 3 corpus and Packaging lint.

| Source package (isolated) | Lines | Functions | Regions | Artifact ID |
| --- | --- | --- | --- | --- |
| `vectorcraft-geom` (10 source files) | **2,037/2,114 = 96.36%** | 314/334 = 94.01% | 5,146/5,384 = 95.58% | `11779066391` |
| `vectorcraft-format` (3 source files) | **583/601 = 97.00%** | 106/111 = 95.50% | 1,486/1,548 = 95.99% | `11778757504` |
| `vectorcraft-engine` (289 source files) | **75,287/77,831 = 96.73%** | 11,325/11,995 = 94.41% | 215,251/223,795 = 96.18% | `11779107961` |
| `vectorcraft-mcp` (20 source files) | **2,515/2,616 = 96.14%** | 320/352 = 90.91% | 7,141/7,513 = 95.05% | `11779197141` |

Engine and MCP LLVM reports are now independently measured; **whole-product and branch-level coverage remain unmeasured**. Do not extrapolate the four percentages to unmeasured crates/UI.

**Exact harness inventory at SHA `95270cf4ef5cbe8cacf8432029da60f86e17b40b`:** `cargo xtask stats --exact` TOTAL reports **5,341 declared**, 40 ignored, 5,306 in its final inventory column. This is not an executed/pass count. Artifact `11780291784`. Full workspace `cargo xtask ci` passed. New remote TCP tests have not yet passed on the latest head.

Do not assign `VERIFIED` based on the presence of tests, nor assert a 100% target before instrumented evidence and justified exclusions.
