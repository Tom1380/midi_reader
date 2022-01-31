mod custom_sounds;

use custom_sounds::*;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

// 36 -> C3
// 37 -> C#3
// ...
// 96 -> C8
// We don't need it now, but it's very handy to keep.
#[allow(dead_code)]
pub fn note_name(note_index: u8) -> String {
    let octave = note_index / 12;
    let remainder = note_index % 12;
    // The 12 notes, octave independent.
    let absolute_notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note = absolute_notes[remainder as usize];
    format!("{note}{octave}")
}

#[derive(Debug)]
pub enum NoteMessage {
    // Note index, velocity
    On(u8, u8),
    // Note index
    Off(u8),
}

struct NotePlayer {
    // We don't use it, but stream_handle needs it to work.
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<u8, Sink>,
    volume: f32,
}

pub fn spawn_note_player(rx: mpsc::Receiver<NoteMessage>) {
    thread::spawn(move || {
        let mut player = NotePlayer::new();
        loop {
            let msg = rx.recv().unwrap();
            match msg {
                NoteMessage::On(note_index, velocity) => {
                    player.note_on(note_index, velocity);
                }
                NoteMessage::Off(note_index) => {
                    player.note_off(note_index);
                }
            }
        }
    });
}

impl NotePlayer {
    fn new() -> NotePlayer {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        NotePlayer {
            _stream,
            stream_handle,
            sinks: HashMap::new(),
            volume: 0.1,
        }
    }

    fn note_on(&mut self, note_index: u8, velocity: u8) {
        match self.sinks.get(&note_index) {
            Some(_) => {}
            None => {
                self.sinks
                    .insert(note_index, self.build_sink(note_index, velocity));
            }
        }
    }

    fn build_sink(&self, note_index: u8, velocity: u8) -> Sink {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(self.volume * velocity as f32 / 100.0);
        let sound_wave = Self::get_sound_wave(note_index);
        sink.append(sound_wave);
        sink
    }

    fn note_off(&mut self, note_index: u8) {
        match self.sinks.remove(&note_index) {
            Some(sink) => sink.stop(),
            None => {}
        }
    }

    fn get_sound_wave(note_index: u8) -> SawWave {
        SawWave::new(Self::get_frequency(note_index))
    }

    // C1: 32hz
    fn get_frequency(note_index: u8) -> f32 {
        // TODO check this is accurate.
        32.0 * (2.0_f32).powf(1.0 / 12.0_f32).powf(note_index as f32)
    }
}
