use midi::note::Note;

fn str_to_note(n: &str) -> Option<Note> {
    match n {
        // A
        "a3" => Some(Note::A3),
        "a4" => Some(Note::A4),
        "a5" => Some(Note::A5),
        "a6" => Some(Note::A6),
        "a7" => Some(Note::A7),
        "a8" => Some(Note::A8),
        // A#
        "a#0" => Some(Note::As0),
        "a#1" => Some(Note::As1),
        "a#2" => Some(Note::As2),
        "a#3" => Some(Note::As3),
        "a#4" => Some(Note::As4),
        "a#5" => Some(Note::As5),
        "a#6" => Some(Note::As6),
        "a#7" => Some(Note::As7),
        "a#8" => Some(Note::As8),
        // B
        "b0" => Some(Note::B0),
        "b1" => Some(Note::B1),
        "b2" => Some(Note::B2),
        "b3" => Some(Note::B3),
        "b4" => Some(Note::B4),
        "b5" => Some(Note::B5),
        "b6" => Some(Note::B6),
        "b7" => Some(Note::B7),
        "b8" => Some(Note::B8),
        // C
        "c0" => Some(Note::C0),
        "c1" => Some(Note::C1),
        "c2" => Some(Note::C2),
        "c3" => Some(Note::C3),
        "c4" => Some(Note::C4),
        "c5" => Some(Note::C5),
        "c6" => Some(Note::C6),
        "c7" => Some(Note::C7),
        "c8" => Some(Note::C8),
        // C#
        "c#0" => Some(Note::Cs0),
        "c#1" => Some(Note::Cs1),
        "c#2" => Some(Note::Cs2),
        "c#3" => Some(Note::Cs3),
        "c#4" => Some(Note::Cs4),
        "c#5" => Some(Note::Cs5),
        "c#6" => Some(Note::Cs6),
        "c#7" => Some(Note::Cs7),
        "c#8" => Some(Note::Cs8),
        // D
        "d0" => Some(Note::D0),
        "d1" => Some(Note::D1),
        "d2" => Some(Note::D2),
        "d3" => Some(Note::D3),
        "d4" => Some(Note::D4),
        "d5" => Some(Note::D5),
        "d6" => Some(Note::D6),
        "d7" => Some(Note::D7),
        "d8" => Some(Note::D8),
        // D#
        "d#0" => Some(Note::Ds0),
        "d#1" => Some(Note::Ds1),
        "d#2" => Some(Note::Ds2),
        "d#3" => Some(Note::Ds3),
        "d#4" => Some(Note::Ds4),
        "d#5" => Some(Note::Ds5),
        "d#6" => Some(Note::Ds6),
        "d#7" => Some(Note::Ds7),
        "d#8" => Some(Note::Ds8),
        // E
        "e0" => Some(Note::E0),
        "e1" => Some(Note::E1),
        "e2" => Some(Note::E2),
        "e3" => Some(Note::E3),
        "e4" => Some(Note::E4),
        "e5" => Some(Note::E5),
        "e6" => Some(Note::E6),
        "e7" => Some(Note::E7),
        "e8" => Some(Note::E8),
        // F
        "f0" => Some(Note::F0),
        "f1" => Some(Note::F1),
        "f2" => Some(Note::F2),
        "f3" => Some(Note::F3),
        "f4" => Some(Note::F4),
        "f5" => Some(Note::F5),
        "f6" => Some(Note::F6),
        "f7" => Some(Note::F7),
        "f8" => Some(Note::F8),
        // F#
        "f#0" => Some(Note::Fs0),
        "f#1" => Some(Note::Fs1),
        "f#2" => Some(Note::Fs2),
        "f#3" => Some(Note::Fs3),
        "f#4" => Some(Note::Fs4),
        "f#5" => Some(Note::Fs5),
        "f#6" => Some(Note::Fs6),
        "f#7" => Some(Note::Fs7),
        "f#8" => Some(Note::Fs8),
        // G
        "g0" => Some(Note::G0),
        "g1" => Some(Note::G1),
        "g2" => Some(Note::G2),
        "g3" => Some(Note::G3),
        "g4" => Some(Note::G4),
        "g5" => Some(Note::G5),
        "g6" => Some(Note::G6),
        "g7" => Some(Note::G7),
        "g8" => Some(Note::G8),
        _ => None
    }
}

pub fn to_notes(s: Vec<String>) -> Vec<Note> {
    let mut v = Vec::with_capacity(s.len());
    for c in s {
        let n = str_to_note(&c.to_lowercase());
        match n {
            Some(note) => v.push(note),
            None => { }
        }
    }
    v
}
