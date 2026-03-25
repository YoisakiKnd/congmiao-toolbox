# Congmiao Toolbox

Congmiao Toolbox is a local desktop toolbox built with Tauri 2, Svelte 5, and Bun.

## Features

- Desktop utility hub with multiple local tools in a single window
- Peek PC LAN monitoring with blurred screenshot output and foreground window status
- Screen time tracking
- Heart rate overlay / widget support
- In-app updater and launch-at-login support

## Stack

- Tauri 2
- Svelte 5
- TypeScript
- Bun
- Rust

## Local Development

```bash
bun install
bun run tauri:dev
```

## Production Build

```bash
bun run tauri:build
```

## Release Workflow

Version releases are published from GitHub Actions when you push a tag in the form `v*`.

```bash
git tag v0.1.3
git push origin v0.1.3
```

The current workflow publishes:

- macOS build
- Windows NSIS `.exe` build

Windows MSI is intentionally disabled.

## Updater Secrets

The updater workflow requires these GitHub repository secrets:

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Generate the updater signing key locally on your own machine, then copy the key content and password into the GitHub repository secrets above.

Do not commit the private key, password file, or any machine-specific local path into this repository.
