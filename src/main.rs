#![feature(slice_patterns)]
#[macro_use]
extern crate clap;
extern crate biot;

use clap::{App, SubCommand};
use std::process::exit;
use biot::pattern_count;

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
