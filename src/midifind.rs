extern crate nom_midi as midi;
extern crate nom;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use midi::MidiEventType;
use midi::MidiEvent;
use midi::EventType;
use midi::note::Note;

pub fn is_good_midi(pth: &Path, notes: &[Note]) -> bool {
    let file = File::open(pth).unwrap();
    let mut buffer = Vec::new();
    let mut reader = io::BufReader::new(file);
    if let Err(e) = reader.read_to_end(&mut buffer) {
        println!("Failed to read file: {:?}", e);
        return false
    }

    let midi_parsed = midi::parser::parse_midi(&buffer);

    let mut index = 0;

    if let nom::IResult::Done(_, midi_data) = midi_parsed {
        for t in midi_data.tracks {
            for e in t.events {
                match e {
                    midi::Event { event: EventType::Midi( MidiEvent { event: MidiEventType::NoteOn(n, _), .. }), .. } => {
                        if n == notes[index] {
                            index += 1;
                            if index == notes.len() {
                                return true
                            }
                        } else {
                            index = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return false;
}
