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
use biot::*;

fn main() {
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  match matches.subcommand {
    Some(cmd) => match cmd.as_ref() {
      &SubCommand{ref name, ref matches} => match name.as_ref() {
        "frequent_words" => {
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
          let l1 = lines.next().unwrap().unwrap();
          let l2 = lines.next().unwrap().unwrap();
          let v: Vec<&str> = l2.split(' ').collect();

          let k = v[0].parse().unwrap();
          let d = v[1].parse().unwrap();

          println!("{:?}", frequent_words_with_mismatches(&l1, k, d));

        },
        "count" => {
          let text = &matches.args["text"].vals[0];
          let text_string = text.to_str().unwrap();
          let pattern = &matches.args["pattern"].vals[0];
          let pattern_string = pattern.to_str().unwrap();
          let distance = match matches.value_of("distance") {
            Some(n) => match n.parse() {
              Ok(n) => n,
              Err(_) => panic!("\"{}\" is not a valid number", n)
            },
            None => 0
          };

          println!("{}", approx_pattern_count(&text_string, &pattern_string, distance));
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

          let index = match matches.value_of("index") {
            Some(n) => match n.parse() {
              Ok(n) => n,
              Err(_) => panic!("\"{}\" is not a valid number", n)
            },
            None => 0
          };
          let file = &matches.args["file"].vals[0];
          let t = &matches.args["type"].vals[0];
          let f = match t.to_str().unwrap() {
            "max" => max_skew,
            "min" => min_skew,
            _ => panic!("Only types \"min\" and \"max\" are valid")
          };

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
            Ok(_) => {
              let displaced = &contents[index..];
              let rest = &contents[..index];
              let mut str = String::from(rest);
              str.push_str(displaced);
              println!("{:?}", f(&contents))
            }
          }
        },
        _ => exit(0)
      },
    },
    _ => exit(0)
  }
}
