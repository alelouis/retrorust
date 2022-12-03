extern crate midir;

use midir::{MidiInput, MidiInputPort};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::Sender;

/// Compute the frequency in Hz for a given midi note
fn midi_key_to_frequency(key: u8) -> f32 {
    440. * 2.0_f32.powf((key as f32 - 69.) / 12.)
}

/// Prompt for midi input choice
fn choose_input_port(in_ports: &Vec<MidiInputPort>, midi_in: &MidiInput) -> Option<usize> {
    // Display all ports
    println!("\nAvailable input ports:");
    for (i, p) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p).unwrap());
    }

    // Select prompt
    print!("Please select input port: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // Return index
    match input.trim().parse::<usize>() {
        Ok(index) => Some(index),
        _ => None,
    }
}

/// Opens MIDI connection and stream inputs into channel rx
pub fn stream(tx: Sender<f32>) {
    // Declare midi input
    let midi_in = MidiInput::new("midir reading input").unwrap();

    // Get input ports
    let in_ports = midi_in.ports();

    // Prompt input port to use
    let in_port_index = choose_input_port(&in_ports, &midi_in).unwrap();
    let in_port = &in_ports[in_port_index];

    // Show current choice
    let in_port_name = midi_in.port_name(in_port).unwrap();
    println!("Opened MIDI input {in_port_name}");

    // Connect to port and stream keys
    let _conn_in = midi_in
        .connect(
            in_port,
            "midir-read-input",
            move |stamp, message, _| {
                if message[0] == 0x90 {
                    tx.send(midi_key_to_frequency(message[1])).unwrap();
                }
            },
            (),
        )
        .unwrap();

    loop {}
}
