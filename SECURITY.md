# Security policy

Restnaa is pre-release software. Only the latest source/candidate is actively reviewed; there is no promised security-support period or response SLA.

## Reporting a vulnerability

Do not post exploit details, secrets, databases or personal diagnostics in a public issue. Use the repository's Security ? Advisories ? Report a vulnerability control when private reporting is enabled. If it is unavailable, open a minimal issue requesting a private contact without disclosing the vulnerability. No private inbox or enabled GitHub reporting channel is claimed before the repository exists.

Include affected version, Windows version, a minimal reproduction, impact and suggested mitigation. Use synthetic data. Reports are handled on a best-effort basis.

## Boundaries

The frontend has narrowly scoped Tauri capabilities and no direct SQL/filesystem access. Rust validates input and owns scheduling/storage. Corrupt databases must fail visibly without silent reset. Local SQLite is not encrypted by this app. Windows notifications and embedded WebView2 have their own privacy/security behavior; see PRIVACY.md.

Maintainers must enable private vulnerability reporting after creating the public repository, review access and protect signing credentials outside Git. Never add CI secrets to pull-request code execution. See docs/OPEN_SOURCE.md for publication steps.
