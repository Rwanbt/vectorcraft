# Residual production-readiness risks

**Assessed:** 2026-10-13, `main@436d7ecdd37298503ee1bb86d0349f9b7b2dc896`.

1. **BLOCKER — latest-SHA TCP command delivery hardening not yet verified.** All ten Linux/Windows/macOS quality jobs and the Affinity/Packaging workflows passed at `95270cf4ef5cbe8cacf8432029da60f86e17b40b`; the subsequent no-automatic-replay fix and real TCP tests require a new run.
2. **HIGH — no whole-product instrumented coverage.** Isolated line coverage verified at checkpoint: geom 96.36%, format 97.00%, engine 96.73%, MCP 96.14%. All other crates, branch coverage and justified exclusions still require measurement; never aggregate incompatible counters.
3. **HIGH — desktop/web full user-journey E2E not demonstrated.** Real app launch, input/edit/undo, save/reload, SVG/PDF/raster export, browser responsive behavior and installation/uninstallation need dedicated repeatable tests and runners.
4. **HIGH — untrusted inputs and data-loss paths.** Existing format fuzzing, root confinement and native-save scenarios passed at the verified checkpoint. MCP protocol framing and strict-version tests passed at checkpoint; new TCP acknowledgement/replay regressions still need validation; plug-ins, more file decoders and broader persistence scenarios remain out of scope.
5. **HIGH — portability.** New geometry tests are scoped to Windows/macOS; full-app native validation and usable GPU/headless UI runners remain outside this first slice.
6. **MEDIUM — upstream divergence.** The upstream was 349 commits ahead of the fork base at audit; re-evaluate each functional change before any future sync. **No automatic merge**.
7. **MEDIUM — deterministic fuzzing.** Existing `prop_geom.rs` uses property-based tests with `failure_persistence: None`. New metamorphic tests fix seed 20261012 for reproducible CI; additionally record any failures in unseeded runs and keep minimized regression cases. Fixed seeds supplement, not replace, broad fuzzing.

**Quality decision:** NOT PRODUCTION READY on the evidence available at the start of this branch; no PR should leave draft without exact-SHA green CI and coverage evidence. Completion of this slice is not completion of the application-wide mission.
