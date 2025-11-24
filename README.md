# Bose

Command-line control for your [Bose SoundTouch 20](https://www.bosebelgium.be/nl_be/support/products/bose_speakers_support/bose_smarthome_speakers_support/soundtouch-20-wireless-music-system.html) speaker, built on top of the [`bose_soundtouch`](https://github.com/timvw/bose_soundtouch) library. The CLI is now a thin async wrapper around the reusable crate, so it always benefits from the library’s protocol coverage, fixes, and optional features.

## Relationship to `bose_soundtouch`

- `bose_soundtouch` is the published Rust library that implements the entire HTTP/WebSocket SoundTouch API surface.
- This repository provides a ready-to-use binary that depends on `bose_soundtouch` (via a path dependency in development or the crates.io release in production).
- If you are writing Rust code that needs to integrate with SoundTouch devices directly, depend on `bose_soundtouch` in your project; if you just need a CLI, install/use this repo.

## Installation

```bash
brew install timvw/tap/bose
```

This pours the prebuilt universal (arm64 + x86_64) binary from
[`timvw/homebrew-tap`](https://github.com/timvw/homebrew-tap), so you can start
using the CLI without installing Rust locally. Keep it current with `brew update
&& brew upgrade bose` whenever a new release lands.

## Usage

```bash
bose
Control your Bose SoundTouch 20

Usage: bose [OPTIONS] <COMMAND>

Commands:
  status  Print the current status
  power   Press (and release) the power button
  play    Press (and release) the play button
  pause   Press (and release) the pause button
  volume  Get and set the volume
  preset  Get and set a preset
  help    Print this message or the help of the given subcommand(s)

Options:
      --hostname <HOSTNAME>  Hostname of the Bose system [env: HOSTNAME=] [default: bose-woonkamer.local]
  -h, --help                 Print help
  -V, --version              Print version
```

Turn the system on/off:

```bash
bose power
```

Select preset #2:

```bash
bose preset 3
```

Set the volume to 5:
```bash
bose volume 5
```

## Development

The CLI uses the local checkout of `bose_soundtouch` via a `path` dependency. When publishing or building elsewhere, switch the dependency to the crates.io version if desired. Run `cargo check`/`cargo run` as usual; Tokio powers the async main function that forwards all actions to `BoseClient`.

## Release automation

Tagged releases are built and published by the `Release` GitHub Actions workflow:

- Create a semver tag (for example `v0.2.1`) and push it to GitHub: `git tag v0.2.1 && git push origin v0.2.1`.
- The workflow builds Linux (`x86_64-unknown-linux-gnu`), macOS (universal `aarch64/x86_64`), and Windows (`x86_64-pc-windows-msvc`) binaries in release mode.
- Each build is packaged with the README and license so downstream taps can consume a single archive per platform (tarballs for macOS/Linux, zip for Windows).
- After all jobs finish, the artifacts are uploaded to the GitHub release matching the tag, ready for Homebrew or other package automation.
