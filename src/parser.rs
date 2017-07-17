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
    match captures {
        None => Err("no match"),
        _ => {
            let caps = captures.unwrap();
            let minutes = &caps["minute"].parse::<f32>().unwrap();
            let (sec, endf) = (&caps["sec"], &caps["endf"]);
            let sec_msec = [sec, endf].join(".").parse::<f32>().unwrap();
            let str_seconds:String = format!("{}", ((((minutes * 60.0) + sec_msec)) * 24000.0).round());
            Ok(str_seconds)
        }
    }
}

pub fn convert_to_fcpxml_time(time: &str) -> String {
    [&convert_to_seconds(time).unwrap(), "/24000s"].join("")
}

pub fn fcpxml_duration(start_time: u32, end_time: u32) -> String {
    format!("{}/120000s", (((end_time - start_time) / 24000) * 120000))
}

// pad for fcpxml timeline position
pub fn fcpxml_pad(time: &str, fcpxml_pad:i32) -> i32 {
    (convert_to_seconds(time).unwrap().parse::<i32>().unwrap() - fcpxml_pad)
}

pub fn convert_msec(second: i32) -> i32 {
    ((second as f32) / 3.92) as i32
}

pub fn strip_format(line: &str) -> String {
    let re = Regex::new(r"\{[^}]*\}").unwrap();
    re.replace_all(line, "").to_string()
}
