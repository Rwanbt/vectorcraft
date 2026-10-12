# Test execution and evidence

**Repo:** `Rwanbt/vectorcraft` — **base SHA:** `436d7ecdd37298503ee1bb86d0349f9b7b2dc896`
**Branch:** `test/fork-quality-baseline-20261012`
**Baseline date:** 2026-10-12

The GitHub fork had **zero workflow runs** when the audit began. Shell access here cannot clone GitHub (DNS unavailable), so the first actual Rust execution must be obtained from the PR's GitHub Actions results. Until then, the tests below are **NOT EXECUTED**, not passed.

| GitHub Actions job | Intended validation | Current evidence |
| --- | --- | --- |
| Linux - complete workspace quality gate | canonical `cargo xtask ci`: rustfmt, Clippy, workspace tests, assets, brand, layers, WASM | No run checked yet |
| Linux - instrumented geometry baseline | actual `cargo llvm-cov` geometry unit+integration tests, LCOV and JSON artifacts | No run checked yet |
| Geometry regression - windows-latest | `cargo test -p vectorcraft-geom --locked` | No run checked yet |
| Geometry regression - macos-latest | `cargo test -p vectorcraft-geom --locked` | No run checked yet |

When the PR workflow starts, record its **head SHA**, run URL, all job results, the geometry summary, and any failure logs here. Distinguish blocked runner/toolchain configuration from failing tests. Do not reuse an older run after a commit changes the SHA.

The complete product-level E2E matrix (GUI desktop, browser/WASM interaction, package lifecycle, import/edit/undo/save/reopen/export) is not satisfied by the new geometry gate.
