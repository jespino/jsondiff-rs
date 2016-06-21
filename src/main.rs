#![feature(test)]
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate test;

mod diff;
mod lcs;

use clap::{Arg, App};
use serde_json::ser::to_string_pretty;
use serde_json::de::from_reader;
use std::io::prelude::*;
use std::fs::File;
use diff::compare;

fn main() {
    let matches = App::new("JsonDiff")
                          .version("1.0")
                          .author("Jesús Espino <jespinog@gmail.com>")
                          .about("Generate the difference between 2 json")
                          .arg(Arg::with_name("INPUT1")
                               .help("Sets the first input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("INPUT2")
                               .help("Sets the second input file to use")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("OUTPUT")
                               .help("Sets the output file to use")
                               .required(false)
                               .index(3))
                          .get_matches();

	let input1 = matches.value_of("INPUT1").unwrap();
    println!("loading file {}", input1);
    let data1 = from_reader(&mut File::open(input1).unwrap()).unwrap();

	let input2 = matches.value_of("INPUT2").unwrap();
    println!("loading file {}", input2);
    let data2 = from_reader(&mut File::open(input2).unwrap()).unwrap();

    println!("processing");
    let differences = compare(&data1, &data2);
    match matches.value_of("OUTPUT") {
        Some(output) => { let _ = File::create(output).unwrap().write_all(to_string_pretty(&differences).unwrap().as_bytes()); },
        None => { println!("{}", to_string_pretty(&differences).unwrap()); }
    }
}
