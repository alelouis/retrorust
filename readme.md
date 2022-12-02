# **retrorust**
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)

Mini retro-vibe monophonic synthesizer.

Generates audio PWM square waves using [NES APU-like](https://www.nesdev.org/wiki/APU) implementation and visualize resulting waveform in real-time.

https://user-images.githubusercontent.com/6841652/205342698-4e292fd3-3663-4e95-b6ab-5192d580bb4f.mov

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
