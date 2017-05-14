#![feature(slice_patterns)]
#[macro_use]
extern crate clap;

use clap::{App, SubCommand};
use std::process::exit;

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
                _ => exit(0)
            },
        },
        _ => exit(0)
    }
}


pub fn pattern_count(text: &str, pattern: &str) -> i32 {
    let mut count: i32 = 0;

    for i in 0..(text.len() - pattern.len() + 1) {
        let slice = &text[i..(i + pattern.len())];
        if slice == pattern {
            count += 1;
        }
    }

    return count;
}
