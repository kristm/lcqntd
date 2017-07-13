extern crate fcpxmlgen;

use fcpxmlgen::parser::parse_line;

static SIMPLE_LINE: &'static str = "Dialogue: 0,0:11:26.40,0:11:27.64,Default,,0,0,0,,Don't you hang the phone";
static VERBOSE_LINE: &'static str = "Dialogue: 0,0:00:28.00,0:00:36.89,Default,,0,0,0,,{\\move(427,470,427,470,28,-14)}Hello Good Afternoon, Is Happy Around?";
static ACCENTED_LINE: &'static str = "Dialogue: 0,0:03:01.40,0:03:23.01,Default,,0,0,0,,Esta mañana escuche en el jardín de tu casa";

#[test]
fn it_matches_dialogue() {
    assert_eq!(&parse_line(SIMPLE_LINE)["dialogue"], "Don't you hang the phone");
    assert_eq!(&parse_line(VERBOSE_LINE)["dialogue"], "Hello Good Afternoon, Is Happy Around?");
    assert_eq!(&parse_line(ACCENTED_LINE)["dialogue"], "Esta mañana escuche en el jardín de tu casa");
}

#[test]
fn it_matches_start_time() {
    assert_eq!(&parse_line(SIMPLE_LINE)["starttime"], "0:11:26.40");
    assert_eq!(&parse_line(VERBOSE_LINE)["starttime"], "0:00:28.00");
    assert_eq!(&parse_line(ACCENTED_LINE)["starttime"], "0:03:01.40");
}

#[test]
fn it_matches_end_time() {
    assert_eq!(&parse_line(SIMPLE_LINE)["endtime"], "0:11:27.64");
    assert_eq!(&parse_line(VERBOSE_LINE)["endtime"], "0:00:36.89");
    assert_eq!(&parse_line(ACCENTED_LINE)["endtime"], "0:03:23.01");
}
