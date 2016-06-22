#![feature(test)]
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate test;
extern crate jsondiff;

#[macro_use]
extern crate log;
extern crate loggerv;

use clap::{Arg, App};
use serde_json::ser::to_string_pretty;
use serde_json::de::from_reader;
use std::io::prelude::*;
use std::fs::File;
use jsondiff::{diff, similarity};

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
                          .arg(Arg::with_name("similarity")
                               .short("s")
                               .long("similarity")
                               .help("Show only the similarity"))
						  .arg(Arg::with_name("verbosity")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .arg(Arg::with_name("output")
                               .short("o")
                               .long("output")
                               .value_name("FILE")
                               .help("Sets the output file to use")
                               .takes_value(true))
                          .get_matches();

	loggerv::init_with_verbosity(matches.occurrences_of("verbosity")).unwrap();

	let input1 = matches.value_of("INPUT1").unwrap();
    info!("loading file {}", input1);
    let data1 = match input1 {
        "-" => from_reader(std::io::stdin()).unwrap(),
        value => from_reader(File::open(value).unwrap()).unwrap()
    };

	let input2 = matches.value_of("INPUT2").unwrap();
    info!("loading file {}", input2);
    let data2 = match input2 {
        "-" => from_reader(std::io::stdin()).unwrap(),
        value => from_reader(File::open(value).unwrap()).unwrap()
    };

    info!("processing");
    if matches.is_present("similarity") {
        println!("{:.2}%", similarity(&data1, &data2));
    } else {
        let differences = diff(&data1, &data2);
        match matches.value_of("output") {
            Some("-") => { let _ = std::io::stdout().write_all(to_string_pretty(&differences).unwrap().as_bytes()); },
            Some(output) => { let _ = File::create(output).unwrap().write_all(to_string_pretty(&differences).unwrap().as_bytes()); },
            None => { println!("{}", to_string_pretty(&differences).unwrap()); }
        }
    }
}
