# Residual production-readiness risks

**Assessed:** 2026-10-12, `main@436d7ecdd37298503ee1bb86d0349f9b7b2dc896`.

1. **BLOCKER — complete latest-SHA quality gate not yet green.** New Linux/Windows/macOS jobs exist; an earlier geometry-only run passed, but the latest full-workspace gate still needs verification. Review the exact PR head SHA.
2. **HIGH — no full-workspace instrumented coverage.** Initial instrumentation only measures `vectorcraft-geom`. Extend per-crate line/function/branch measurements after stabilizing the baseline. Never aggregate incompatible counters.
3. **HIGH — desktop/web full user-journey E2E not demonstrated.** Real app launch, input/edit/undo, save/reload, SVG/PDF/raster export, browser responsive behavior and installation/uninstallation need dedicated repeatable tests and runners.
4. **HIGH — untrusted inputs and data-loss paths.** Existing tests address format fuzzing and root confinement, but their execution and breadth across file decoders, MCP, plug-ins and persistence remain unverified.
5. **HIGH — portability.** New geometry tests are scoped to Windows/macOS; full-app native validation and usable GPU/headless UI runners remain outside this first slice.
6. **MEDIUM — upstream divergence.** The upstream was 349 commits ahead of the fork base at audit; re-evaluate each functional change before any future sync. **No automatic merge**.
7. **MEDIUM — deterministic fuzzing.** Existing `prop_geom.rs` uses property-based tests with `failure_persistence: None`. New metamorphic tests fix seed 20261012 for reproducible CI; additionally record any failures in unseeded runs and keep minimized regression cases. Fixed seeds supplement, not replace, broad fuzzing.

**Quality decision:** NOT PRODUCTION READY on the evidence available at the start of this branch; no PR should leave draft without exact-SHA green CI and coverage evidence. Completion of this slice is not completion of the application-wide mission.
