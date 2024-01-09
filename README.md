# Bose

Control your [Bose SoundTouch 20](https://www.bosebelgium.be/nl_be/support/products/bose_speakers_support/bose_smarthome_speakers_support/soundtouch-20-wireless-music-system.html) speaker from the command line.

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
