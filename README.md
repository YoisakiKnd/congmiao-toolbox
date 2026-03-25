# Congmiao Toolbox

A local desktop toolbox starter built with Tauri, React, Material UI, and Bun.

## Stack

- Tauri 2
- React 19
- Material UI 7
- Bun
- Vite

## Local Development

```bash
bun install
bun run tauri:dev
```

## Production Build

```bash
bun run tauri:build
```

## Release Automation

GitHub Actions is configured to build and publish release artifacts for:

- macOS Apple Silicon (`macos-latest`)
- Windows x64 NSIS (`windows-latest`, `.exe` only)

Create a tag like `v0.1.0` and push it to publish a signed updater release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Before the updater workflow can publish installable updates, add these GitHub repository secrets:

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

This workspace already generated a local updater keypair for you at:

- `/Users/tenonsuzu/.tauri/congmiao-toolbox-updater.key`
- `/Users/tenonsuzu/.tauri/congmiao-toolbox-updater.password`

Manual `workflow_dispatch` runs produce draft preview releases, while version tags publish the real updater feed.

The workflow file lives at `.github/workflows/release.yml`.
