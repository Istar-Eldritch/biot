#![feature(slice_patterns)]
#[macro_use]
extern crate clap;
extern crate biot;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{App, SubCommand};
use std::process::exit;
use biot::{pattern_count, min_skew, hamming_distance};

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand {
        Some(cmd) => match cmd.as_ref() {
            &SubCommand{ref name, ref matches} => match name.as_ref() {
                "count" => {
                    let text = &matches.args["text"].vals[0];
                    let text_string = text.to_str().unwrap();
                    let pattern = &matches.args["pattern"].vals[0];
                    let pattern_string = pattern.to_str().unwrap();
                    println!("{}", pattern_count(&text_string, &pattern_string));
                },
                "hamming_distance" => {
                    let file = &matches.args["file"].vals[0];
                    let file_path: &str = file.to_str().unwrap();
                    let path = Path::new(file_path);
                    let display = path.display();

                    let file = match File::open(&path) {
                        // The `description` method of `io::Error` returns a string that
                        // describes the error
                        Err(_) => panic!("couldn't open {}", display),
                        Ok(file) => file,
                    };

                    let buf_reader = BufReader::new(file);

                    let mut lines = buf_reader.lines();
                    let t1 = lines.next().unwrap().unwrap();
                    let t2 = lines.next().unwrap().unwrap();

                    println!("{}", hamming_distance(&t1, &t2));
                },
                "skew" => {

                    let file = &matches.args["file"].vals[0];
                    let file_path: &str = file.to_str().unwrap();
                    let path = Path::new(file_path);
                    let display = path.display();

                    let file = match File::open(&path) {
                        // The `description` method of `io::Error` returns a string that
                        // describes the error
                        Err(_) => panic!("couldn't open {}", display),
                        Ok(file) => file,
                    };

                    let mut buf_reader = BufReader::new(file);
                    let mut contents = String::new();
                    match buf_reader.read_to_string(&mut contents) {
                        Err(_) => panic!("couldn't read {}", display),
                        Ok(_) => println!("{:?}", min_skew(&contents))
                    }
                },
                _ => exit(0)
            },
        },
        _ => exit(0)
    }
}
