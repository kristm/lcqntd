use regex::Regex;
use regex::Captures;
// 
// enum Subs {
//     Dialogue(String)
// }

pub fn parse_line(line: &str) -> Captures {
    let re = Regex::new(r"Default,,[\d,]+(?P<dialogue>[^{}]*)$").unwrap();
    let captures = re.captures(line).unwrap();
    captures
}

