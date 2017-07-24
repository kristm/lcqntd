extern crate lcqntd;
extern crate regex;

use lcqntd::parser::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // Check if file is valid
        let file = match File::open(&arg1) {
            Err(why) => panic!("couldn't open {}: {}", arg1, why.description()),
            Ok(file) => file,
        };


        let re = Regex::new(r"(.*)\.[a-zA-Z]*$").unwrap();
        let output: String = re.replace(&arg1, "$1.fcpxml").to_string();
        let mut writer = BufWriter::new(io::stdout());

        let mut buf_reader = BufReader::new(&file);
        for line in buf_reader.lines() {
            let l = line.unwrap();
            println!("{}", l);
        }
        //let mut contents = String::new();
        //buf_reader.read_to_string(&mut contents);
        // let re = Regex::new(r"(.*)\.[a-zA-Z]*$").unwrap();
        // let output: String = re.replace(&arg1, "$1.fcpxml").to_string();
        // println!("write to {} ", output);
    }
    // try {
    //     let mut file = File::open(arg1)?;
    //     let mut buf_reader = BufReader::new(file);
    //     let mut contents = String::new();
    // }.or_else(|e|
    //     println!("{}", e)
    // )
}
