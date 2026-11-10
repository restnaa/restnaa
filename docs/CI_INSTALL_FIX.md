# CI dependency-install repair

2026-11-10: public GitHub job 129536689792 for run 42727470648, commit 69f4700, failed at npm ci; later checks were skipped. Reproduced against copied manifests in ignored .tools/ci-install-repro: npm ci --ignore-scripts --no-audit --no-fund failed ETARGET, no matching jsdom@30.1.4. Registry metadata confirms this version is unpublished. Existing lockfile and local installed jsdom are 30.1.3.

Plan: correct package.json jsdom to 30.1.3, keeping the existing lockfile/dependency graph. Verify actual clean npm ci with scripts, then unit/lint/type/build/UI checks using the clean dependencies. No scheduler/native/application behavior or Tauri privileges change. Detailed GitHub step logs were not accessed; diagnosis uses public job metadata plus a same-manifest clean-install reproduction.

GitHub also warns that checkout/setup-node v4 target a deprecated Node 20 action runtime; this is separate from the npm install failure. Review official action releases before any maintenance update. Commit/push the validated fix and monitor public run/job metadata. Do not describe a hosted rerun as passed until its conclusion is observed.


## Clean installation

npm ci --no-audit --no-fund --prefix .tools/ci-install-repro: passed, installed 255 packages, normal install scripts enabled. Only copied package manifests were used for this isolated reproduction. The existing lockfile is unchanged; package.json is corrected to its already locked jsdom version. Runtime sources, Rust dependencies, Tauri configuration and packaging identity are unchanged.


Normal repository npm ci (including install scripts and audit): passed, 255 packages installed. Audit reported 10 moderate advisories; this repair does not run npm audit fix or change the locked graph. Lint/typecheck passed after this clean install. Unit/browser validation and hosted rerun are tracked below. No Rust/native source changed, so local native rebuild is not required for this manifest-only repair; the hosted workflow still runs its complete Rust/build pipeline.


After clean installation: 63 unit tests passed across 14 files; production build passed. Production-only npm audit --omit=dev --json returned zero advisories (the 10 moderate findings are outside the production graph). Full development audit remediation is a separate dependency task. Public job metadata confirms the original run failed at installation, not lint/tests; subsequent workflow steps were skipped.



Final local checks after actual clean npm ci: npm run lint, npm run typecheck, npm test -- --run --maxWorkers=1 (63 tests/14 files), npm run build, and npm run test:ui -- --workers=2 (30 tests, 57.2s) all passed. Browser plugin unavailable; existing Edge/Playwright production-asset fixture used. No UI implementation changed. The lockfile remains byte-for-byte unchanged. Commit is scoped to package.json and this report; unrelated local work is excluded. Hosted status must be read from the new run, not inferred from these local passes.
