# Test execution and evidence

**Repo:** `Rwanbt/vectorcraft` — **base SHA:** `436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Branch:** `test/fork-quality-baseline-20261012`
**Baseline date:** 2026-10-12

The GitHub fork had **zero workflow runs** when the audit began. Shell access here cannot clone GitHub (DNS unavailable), so the first actual Rust execution must be obtained from the PR's GitHub Actions results. The first geometry gates have now actually executed at SHA `8f04b29f8c965b1b43362cebdef3d87b8d106d82`. Keep later commits separate until the newer SHA passes.

| GitHub Actions job | Intended validation | Current evidence |
| --- | --- | --- |
| Linux - complete workspace quality gate | canonical `cargo xtask ci`: rustfmt, Clippy, workspace tests, assets, brand, layers, WASM | Running on latest SHA; first full-workspace run failed on a preexisting Clippy lint (now corrected) |
| Linux - instrumented geometry baseline | actual `cargo llvm-cov` geometry unit+integration tests, LCOV and JSON artifacts | **PASS on 8f04**: 63 unit + 19 property/integration, LLVM artifact 11756761695; corrected reporting scope pending latest SHA |
| Geometry regression - windows-latest | `cargo test -p vectorcraft-geom --locked` | **PASS on 8f04**: 82/82 geometry cases |
| Geometry regression - macos-latest | `cargo test -p vectorcraft-geom --locked` | **PASS on 8f04**: geometry cases, job successful |

When the PR workflow starts, record its **head SHA**, run URL, all job results, the geometry summary, and any failure logs here. Distinguish blocked runner/toolchain configuration from failing tests. Do not reuse an older run after a commit changes the SHA.

The complete product-level E2E matrix (GUI desktop, browser/WASM interaction, package lifecycle, import/edit/undo/save/reopen/export) is not satisfied by the new geometry gate.

## Latest implementation slice

- After SHA `8f04`, added invalid-bend regression and stable midpoint oracle, degenerate Bézier tangent cases, one-cut closed path area/edge preservation, endpoint smoothing, and three fixed-seed metamorphic tests.
- The full workspace gate will collect exact test harness counts as an artifact after its canonical quality checks pass.
- The new geometry coverage job uses package-scoped LLVM reporting and rejects other crates' source filenames in the JSON artifact.
- At the time of this document update, these new cases remain **NOT VERIFIED**. Results must be read from the exact head SHA of PR #1, not its historical green jobs.
