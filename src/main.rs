extern crate lcqntd;

use lcqntd::parser::*;
use std::env;

fn main() {
    let formatted_line:String = String::from("Si, una \"{\\i1}cita{\\i0}.\"");
    if let Some(arg1) = env::args().nth(1) {
        println!("say whut {} : {}", strip_format(&formatted_line), arg1);
    }
}
