# **retrorust**
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)

Mini retro-vibe monophonic synthesizer.

Generates audio PWM square waves using [NES APU-like](https://www.nesdev.org/wiki/APU) implementation and visualize resulting waveform in real-time.

https://user-images.githubusercontent.com/6841652/205342698-4e292fd3-3663-4e95-b6ab-5192d580bb4f.mov

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

## **How to use**
Clone the repository.
```bash
gh clone alelouis/retrorust
```
Then run the main binary.
```bash
cargo run --bin main
```

## **How to use**
For now, keys **F6 to F10** are mapped from A440 to C#554.37.  
Mapping can be modified in `src/audio.rs` inside `react_on_keys()` function.