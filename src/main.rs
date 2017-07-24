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
        let file = File::open(&arg1).expect("couldn't open file");
        let mut contents = String::new();

        // output file
        let re = Regex::new(r"(.*)\.[a-zA-Z]*$").unwrap();
        let target_file: String = re.replace(&arg1, "$1.fcpxml").to_string();
        let output = File::create(target_file).expect("unable to create file");
        let mut output = BufWriter::new(output);

        let mut buf_reader = BufReader::new(&file);
        for line in buf_reader.lines() {
            let l = line.unwrap();
            contents.push_str(&format!("{}\n", l)); // return newline
        }

        output.write_all(contents.as_bytes()).expect("Failed to write data");
    }
}
