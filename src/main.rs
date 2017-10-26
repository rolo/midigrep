extern crate nom_midi as midi;
extern crate nom;
extern crate walkdir;
extern crate rayon;

use std::path::PathBuf;
use std::env;

use rayon::prelude::*;
use walkdir::WalkDir;

mod midifind;
mod str_to_note;


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Supply directory");
        return;
    }
    
    let notes = args.into_iter().skip(2).collect::<Vec<String>>();
    let notes = str_to_note::to_notes(notes);

    if notes.len() == 0 {
        println!("Invalid notes. enter notes as follows: c1 c2 b3 etc.");
        return
    }

    let capacity = 10000;
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
