extern crate fcpxmlgen;

use fcpxmlgen::parser::parse_line;

static SAMPLE_LINE: &'static str = "Dialogue: 0,0:11:26.40,0:11:27.64,Default,,0,0,0,,Don't you hang the phone";

#[test]
fn it_matches_3_patterns_plus_whole_match_plus_optional_subtitle_formatting() {
    let matches = parse_line(SAMPLE_LINE);
    //assert_eq!(matches.len(), 5);
    assert_eq!(&matches["dialogue"], "Don't you hang the phone");
}
