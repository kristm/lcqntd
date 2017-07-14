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
