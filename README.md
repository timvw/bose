# Bose

Command-line control for your [Bose SoundTouch 20](https://www.bosebelgium.be/nl_be/support/products/bose_speakers_support/bose_smarthome_speakers_support/soundtouch-20-wireless-music-system.html) speaker, built on top of the [`bose_soundtouch`](https://github.com/timvw/bose_soundtouch) library. The CLI is now a thin async wrapper around the reusable crate, so it always benefits from the library’s protocol coverage, fixes, and optional features. By default the CLI depends on the crates.io `bose_soundtouch` crate.

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

## Release process

- Bump the crate version before tagging: `cargo set-version X.Y.Z && cargo update -p bose --precise X.Y.Z` (keeps Cargo.toml and Cargo.lock aligned).
- Create and push a matching tag: `git tag vX.Y.Z && git push origin vX.Y.Z` (or provide `tag: vX.Y.Z` when dispatching the Release workflow).
- The GitHub Actions Release workflow validates the tag against crate metadata and fails early if they differ, and it verifies the built binary reports the tagged version.
