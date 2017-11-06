extern crate nom_midi as midi;
extern crate nom;

use colored::*;

use std::io;
use std::io::{Read, Write};
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

#[derive(Debug)]
pub enum MidiNoteOutput {
    Note(Note),
    MatchedNote(Note)
}

pub fn print_notes(pth: &Path, notes: &[Note], print_color: bool) {
    let file = File::open(pth).unwrap();
    let mut buffer = Vec::new();
    let mut reader = io::BufReader::new(file);
    if let Err(e) = reader.read_to_end(&mut buffer) {
        println!("Failed to read file: {:?}", e);
        return
    }

    let midi_parsed = midi::parser::parse_midi(&buffer);

    let mut index = 0;
    let mut glob_index = 0;

    let mut result = Vec::new();
    let mut match_buffer = Vec::new();

    if let nom::IResult::Done(_, midi_data) = midi_parsed {
        for t in midi_data.tracks{
            for e in t.events {
                match e {
                    midi::Event { event: EventType::Midi( MidiEvent { event: MidiEventType::NoteOn(n, _), .. }), .. } => {
                        result.push(MidiNoteOutput::Note(n));
                        glob_index += 1;

                        if n == notes[index] {
                            index += 1;
                            match_buffer.push(MidiNoteOutput::MatchedNote(n));
                            if index >= notes.len() {
                                let mut matches = match_buffer.drain(..).collect::<Vec<MidiNoteOutput>>();
                                let mut x = glob_index - 1;
                                while let Some(e) = matches.pop() {
                                    result[x] = e;
                                    x -= 1;
                                }
                                index = 0;
                            } else if index > notes.len() {
                                index = 0;
                                match_buffer.clear();
                            }
                        } else {
                            match_buffer.clear();
                            index = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let output = if print_color {
        result.iter().map(|note| {
            match note {
                &MidiNoteOutput::Note(n) => format!("{:?} ", n),
                &MidiNoteOutput::MatchedNote(n) => format!("{}", String::from(format!("{:?} ", n)).yellow()),
            }
        }).collect::<String>()
    } else {
        result.iter().map(|note| {
            match note {
                &MidiNoteOutput::Note(n) => format!("{:?} ", n),
                &MidiNoteOutput::MatchedNote(n) => format!("{:?} ", n),
            }
        }).collect::<String>()
    };

    println!("{}", output);
    println!("");
}
