use regex::Regex;
use regex::Captures;
// 
// enum Subs {

pub fn parse_line(line: &str) -> Captures {
    //"((?:\d\d?:){2}\d{2}\.\d{2}),((?:\d\d?:){2}\d{2}\.\d{2}),Default[\d\,]+(?:\{\\?([a-z\d\\]*(?:\([0-9\,\-]*\))?)?\})?([^\{\}]*)$"
    let re = Regex::new(r"(?P<starttime>(\d\d?:){2}\d{2}\.\d{2}),(?P<endtime>(\d\d?:){2}\d{2}\.\d{2}),Default,,[\d,]+(\{\\?([a-z\d\\]*(\([0-9,-]*\))?)?\})?(?P<dialogue>[^\{\}]*)$").unwrap();
    let captures = re.captures(line).unwrap();
    captures
}
