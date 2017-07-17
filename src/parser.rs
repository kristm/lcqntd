use regex::Regex;
use regex::Captures;

pub fn parse_line(line: &str) -> Result<Captures, &str> {
    let re = Regex::new(r"(?P<starttime>(?:\d\d?:){2}\d{2}\.\d{2}),(?P<endtime>(?:\d\d?:){2}\d{2}\.\d{2}),Default,,[\d,]+(?P<formatting>\{\\?(?:[a-z\d\\]*(?:\([0-9,-]*\))?)?\})?(?P<dialogue>[^\{\}]*)$").unwrap();
    let captures = re.captures(line);
    match captures {
        None => Err("no match"),
        _ => Ok(captures.unwrap())
    }
}

// pub fn convert_to_seconds(time: &str) -> u32 {
//     let re = Regex::new(r"^(?P<hour>\d+)\:(?P<minute>\d+)\:(?P<sec>\d+)\.(?P<endf>\d+)$")
//     let captures = re.captures(time);
// }

pub fn convert_msec(second: i32) -> i32 {
    ((second as f32) / 3.92) as i32
}
