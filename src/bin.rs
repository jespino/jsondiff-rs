#![feature(test)]
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate test;
extern crate jsondiff;

#[macro_use]
extern crate log;
extern crate loggerv;

use std::thread;
use clap::{Arg, App};
use serde_json::ser::to_string_pretty;
use serde_json::de::from_reader;
use std::io::prelude::*;
use std::fs::File;
use jsondiff::{diff, similarity};
use std::sync::Arc;


fn main() {
    let matches = Arc::new(App::new("JsonDiff")
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
                          .get_matches());

	loggerv::init_with_verbosity(matches.occurrences_of("verbosity")).unwrap();

    let input1 = Arc::new(matches.value_of("INPUT1").unwrap().to_owned());
    let input2 = Arc::new(matches.value_of("INPUT2").unwrap().to_owned());

    info!("loading file {}", *input1);
    let load1_thread = thread::spawn(move || {
        if *input1 == "-".to_string() {
            from_reader(std::io::stdin()).unwrap()
        } else {
            from_reader(File::open(&*input1).unwrap()).unwrap()
        }
    });

    info!("loading file {}", *input2);
    let load2_thread = thread::spawn(move || {
        if *input2 == "-".to_string() {
            from_reader(std::io::stdin()).unwrap()
        } else {
            from_reader(File::open(&*input2).unwrap()).unwrap()
        }
    });

    let data1 = load1_thread.join().unwrap();
    let data2 = load2_thread.join().unwrap();


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
