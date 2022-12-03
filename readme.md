# **retrorust**
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)

Mini retro-vibe monophonic synthesizer.

Generates audio PWM square waves using [NES APU-like](https://www.nesdev.org/wiki/APU) implementation and visualize resulting waveform in real-time.

## **Implementation**
Main components used to generate a Pulse channel are:
- `envelope.rs` : Manages volume envelope in increasing and decreasing fashion as well as looping.
- `lencounter.rs` : Shutdown channel after specified length.
- `sequencer.rs` : 8 values sequencer with 4 different duty cycles.
- `timer.rs` : Trigger timer for others components.

The Pulse struct in `pulse.rs` uses all components in order to build a PWM channel.

For audio and video real-time playback:
- `audio.rs` : Separate thread use to clock the signal and stream the audio signal.
- `main.rs` : Main thread to display the waveform, communicating with audio thread to fetch signal data.
- `midi.rs` : midi message input handling in separate thread.

## **How to use as standalone**
Clone the repository.
```bash
gh clone alelouis/retrorust
```
Then run the main binary.
```bash
cargo run --bin main
```
Plug-in a midi keyboard and you will be prompted to select it at start of program.

## **How to use as VST**
You can also build a VST version of this synthesizer.  
The library is built from `./src/lib.rs` which integrate the `Pulse` struct with VST host audio and events streams (not using graphics, cpal or midir).  
First, install and then build for you target.
```bash
cargo build --target x86_64-apple-darwin --release
```
Then on OSX, bundle the `.dylib` target with the script `bundle.sh`.
```bash
./bundle.sh retrorust target/x86_64-apple-darwin/release/libretrorust.dylib
```
Then paste the `retrorust.vst` in your VST2 folder, e.g. on macOS:
```bash
cp -r retrorust.vst /Library/Audio/Plug-Ins/VST/
```



