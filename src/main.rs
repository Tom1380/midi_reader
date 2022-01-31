mod sound_playing;

use midir::MidiInput;
use sound_playing::*;
use std::error::Error;
use std::sync::mpsc;

use crate::sound_playing::spawn_note_player;

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

    let (tx, rx) = mpsc::channel();

    spawn_note_player::<SquareWave>(rx);

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", callback, tx)?;

    println!("Connection open, reading input from '{in_port_name}'",);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
    }
}

fn callback(_timestamp: u64, message: &[u8], tx: &mut mpsc::Sender<NoteMessage>) {
    if let [_, note_index, velocity] = message {
        tx.send(match velocity {
            0 => NoteMessage::Off(*note_index),
            _ => NoteMessage::On(*note_index, *velocity),
        })
        .unwrap();
    }
}
