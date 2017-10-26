extern crate nom_midi as midi;
extern crate nom;
extern crate walkdir;
extern crate rayon;
extern crate docopt;
#[macro_use] extern crate serde_derive;

use std::path::PathBuf;
use std::env;

use docopt::Docopt;
use rayon::prelude::*;
use walkdir::WalkDir;

mod midifind;
mod str_to_note;


const USAGE: &'static str = "
midigrep

Usage:
    midigrep <path> <notes>...
    midigrep (-h | --help)
    midigrep (-v | --version)

Example:
    midigrep ~/midi c1 cs2 b4

Options:
    -h --help           Show this screen
    -n --notes          List of consequtive notes to search for
    -v --version        Print version
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: Option<String>,
    arg_notes: Vec<String>,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        let version = env!("CARGO_PKG_VERSION");
        println!("");
        println!("midigrep");
        println!("--------");
        println!("version: {}", version);
        return
    }

    let notes = str_to_note::to_notes(args.arg_notes);
    let capacity = 1000;
    let mut queue = Vec::with_capacity(capacity);

    if let Some(pth) = env::args().nth(1) {
        let walker = WalkDir::new(pth).follow_links(true).into_iter();

        for entry in walker {
            match entry {
                Ok(e) => {
                    if e.path().to_str().unwrap().ends_with(".midi") || e.path().to_str().unwrap().ends_with(".mid")  {
                        let mut path_buffer = PathBuf::new();
                        path_buffer.push(e.path());
                        queue.push(path_buffer);

                        if queue.len() == capacity {
                            queue.par_iter().for_each(|pth| {
                                if midifind::is_good_midi(pth, &notes) {
                                    println!("{:?}", pth);
                                }
                            });
                            queue.clear();
                        }
                        //queue.push(e.path());
                        // if midifind::is_good_midi(e.path(), &notes) {
                        //     println!("{:?}", e.path());
                        // }
                    }
                },
                _=> {}
            }
        }

        queue.par_iter().for_each(|pth| {
            if midifind::is_good_midi(pth, &notes) {
                println!("{:?}", pth);
            }
        });
    }
}
