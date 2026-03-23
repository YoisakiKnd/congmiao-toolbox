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

- macOS Intel (`macos-13`)
- macOS Apple Silicon (`macos-latest`)
- Windows x64 (`windows-latest`)

Create a tag like `v0.1.0` and push it to trigger the release workflow:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow file lives at `.github/workflows/release.yml`.
