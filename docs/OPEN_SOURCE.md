# Publish Restnaa on GitHub

Current project: https://github.com/restnaa/restnaa. It already has its initial source commit. The first-push steps below are retained for new/forked repositories; existing clones should use the normal branch/PR workflow.

The source is MIT licensed. Source publication is separate from approval of stable installers. Review NAME_CORRECTION.md for actual checks and pending release gates.

## Before the first push

1. Review LICENSE and third-party notices. Only upload code/assets you have rights to share. Lucide and dependency licenses remain separate. The generated M1 concept is a historical reference, not an application screenshot or third-party design grant.
2. Inspect `git status --short`, `git diff --cached --stat` and `git diff --cached`. No .tools, node_modules, build outputs, database/WAL files, credentials, local profiles or installer exports should be included. Ignore rules do not remove files already tracked.
3. Use GitHub to create an empty public repository named `restnaa` under your chosen account. Do not initialize it with conflicting README/license files. No owner, repository URL or external publication is assumed by this workspace.
4. With the actual repository URL, run:

```powershell
git init -b main
git add .
git diff --cached --stat
git diff --cached
git commit -m "Prepare Restnaa open-source project"
git remote add origin <YOUR-REPOSITORY-URL>
git push -u origin main
```

Replace the bracketed URL; never paste authentication tokens into remote URLs or source. Configure your Git author locally if needed. This guide does not claim these commands were executed.

## Repository configuration

- Enable Issues, private vulnerability reporting and secret scanning/push protection where available. See [GitHub private reporting instructions](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository).
- Set a maintainer private contact for security/conduct reports. Templates work without invented owner links.
- Require review and passing Windows CI on main; verify the first hosted workflow actually runs. Avoid public stable-release claims based solely on build success.
- Review dependency alerts and lockfile updates. Keep workflow permissions read-only unless a specific release task needs more.
- Tag and publish installers only after the relevant installed/clean-machine gates are satisfied. Old Restnaa artifacts are local historical evidence, not Restnaa binaries.

The current name has not been cleared for trademark/domain availability. No legal clearance, comprehensive dependency-license certification or public stable release is asserted.
