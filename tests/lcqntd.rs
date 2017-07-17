extern crate lcqntd;

use lcqntd::parser::*;

static SIMPLE_LINE: &'static str = "Dialogue: 0,0:11:26.40,0:11:27.64,Default,,0,0,0,,Don't you hang the phone";
static VERBOSE_LINE: &'static str = "Dialogue: 0,0:00:28.00,0:00:36.89,Default,,0,0,0,,{\\move(427,470,427,470,28,-14)}Hello Good Afternoon, Is Happy Around?";
static ACCENTED_LINE: &'static str = "Dialogue: 0,0:03:01.40,0:03:23.01,Default,,0,0,0,,Esta mañana escuche en el jardín de tu casa";
static INVALID_LINE: &'static str = "Invalid line";

#[test]
fn it_matches_5() {
    let matches = parse_line(SIMPLE_LINE).unwrap();
    assert_eq!(matches.len(), 5);
}

#[test]
fn it_matches_dialogue() {
    assert_eq!(&parse_line(SIMPLE_LINE).unwrap()["dialogue"], "Don't you hang the phone");
    assert_eq!(&parse_line(VERBOSE_LINE).unwrap()["dialogue"], "Hello Good Afternoon, Is Happy Around?");
    assert_eq!(&parse_line(ACCENTED_LINE).unwrap()["dialogue"], "Esta mañana escuche en el jardín de tu casa");
}

#[test]
fn it_matches_start_time() {
    assert_eq!(&parse_line(SIMPLE_LINE).unwrap()["starttime"], "0:11:26.40");
    assert_eq!(&parse_line(VERBOSE_LINE).unwrap()["starttime"], "0:00:28.00");
    assert_eq!(&parse_line(ACCENTED_LINE).unwrap()["starttime"], "0:03:01.40");
}

#[test]
fn it_matches_end_time() {
    assert_eq!(&parse_line(SIMPLE_LINE).unwrap()["endtime"], "0:11:27.64");
    assert_eq!(&parse_line(VERBOSE_LINE).unwrap()["endtime"], "0:00:36.89");
    assert_eq!(&parse_line(ACCENTED_LINE).unwrap()["endtime"], "0:03:23.01");
}

#[test]
fn it_matches_format_instructions() {
    assert_eq!(&parse_line(VERBOSE_LINE).unwrap()["formatting"], "{\\move(427,470,427,470,28,-14)}");
}

#[test]
#[should_panic(expected = "no group named 'formatting'")]
fn it_matches_nothing_if_has_no_format_instructions() {
    assert_eq!(&parse_line(SIMPLE_LINE).unwrap()["formatting"], "");
}

#[test]
fn it_returns_error_for_invalid_input() {
    assert_eq!(parse_line(INVALID_LINE).err(), Some("no match"));
    assert!(parse_line(INVALID_LINE).is_err());
}

#[test]
fn it_converts_to_milliseconds() {
    assert_eq!(convert_msec(40), 10);
}

#[test]
fn it_converts_to_seconds() {
    assert_eq!(convert_to_seconds("0:00:28.0").unwrap(), "672000");
}
