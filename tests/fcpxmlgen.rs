extern crate fcpxmlgen;

use fcpxmlgen::parser::parse_line;
use std::string::String;

fn sample_line() -> String {
    "Dialogue: 0,0:11:26.40,0:11:27.64,Default,,0,0,0,,Don't you hang the phone".to_string()
}

#[test]
fn it_matches_3_patterns_plus_whole_match_plus_optional_subtitle_formatting() {
    assert_eq!(parse_line(sample_line()), "y")
}
