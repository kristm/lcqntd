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

pub fn convert_to_seconds(time: &str) -> Result<String, &str> {
    let re = Regex::new(r"^\d+:(?P<minute>\d+):(?P<sec>\d+)\.(?P<endf>\d+)$").unwrap();
    let captures = re.captures(time);
    let mut str_seconds = String::from("0:00:00.00");
    match captures {
        None => Err("no match"),
        _ => {
            let caps = captures.unwrap();
            let (minute, sec, endf) = (&caps["minute"], &caps["sec"], &caps["endf"]);
            str_seconds = format!("0:{}:{}.{}", minute, sec, endf);
            println!(">> {} ", str_seconds);
            Ok(str_seconds)
        }
    }
}

pub fn convert_msec(second: i32) -> i32 {
    ((second as f32) / 3.92) as i32
}
