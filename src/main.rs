extern crate lcqntd;
extern crate regex;

use lcqntd::parser::*;
use std::env;
use regex::Regex;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let re = Regex::new(r"(.*)\.[a-zA-Z]*$").unwrap();
        let output: String = re.replace(&arg1, "$1.fcpxml").to_string();
        println!("write to {} ", output);
    }
}
