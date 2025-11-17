# Bose

Command-line control for your [Bose SoundTouch 20](https://www.bosebelgium.be/nl_be/support/products/bose_speakers_support/bose_smarthome_speakers_support/soundtouch-20-wireless-music-system.html) speaker, built on top of the [`bose_soundtouch`](https://github.com/timvw/bose_soundtouch) library. The CLI is now a thin async wrapper around the reusable crate, so it always benefits from the library’s protocol coverage, fixes, and optional features.

## Relationship to `bose_soundtouch`

- `bose_soundtouch` is the published Rust library that implements the entire HTTP/WebSocket SoundTouch API surface.
- This repository provides a ready-to-use binary that depends on `bose_soundtouch` (via a path dependency in development or the crates.io release in production).
- If you are writing Rust code that needs to integrate with SoundTouch devices directly, depend on `bose_soundtouch` in your project; if you just need a CLI, install/use this repo.

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
