use midir::MidiInput;
use std::error::Error;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("midir reading input")?;
    let in_ports = midi_in.ports();
    let in_port = &in_ports[1];
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", callback, ())?;

    println!("Connection open, reading input from '{in_port_name}'",);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
    }
}

fn callback(_timestamp: u64, message: &[u8], _additional_data: &mut ()) {
    if let [_, note_index, velocity] = message {
        let note = note_name(*note_index);
        if *velocity == 0 {
            println!("Note {note} off.");
        } else {
            println!("Note {note} on.");
        }
    }
}

// 36 -> C3
// 37 -> C#3
// ...
// 96 -> C8
fn note_name(note_index: u8) -> String {
    let octave = note_index / 12;
    let remainder = note_index % 12;
    // The 12 notes, octave independent.
    let absolute_notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note = absolute_notes[remainder as usize];
    format!("{note}{octave}")
}
