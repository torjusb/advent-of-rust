#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::collections::HashMap;

fn parse_date(line: &str) -> NaiveDateTime {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]").unwrap();
    }
    let capture = RE.captures(line).unwrap();

    // There must be a better way to do this..
    let year: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
    let month: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
    let day: u32 = capture.get(3).unwrap().as_str().parse().unwrap();
    let hour: u32 = capture.get(4).unwrap().as_str().parse().unwrap();
    let minute: u32 = capture.get(5).unwrap().as_str().parse().unwrap();

    let d = NaiveDate::from_ymd(year, month, day);
    let t = NaiveTime::from_hms(hour, minute, 0);

    NaiveDateTime::new(d, t)
}

fn parse_guard_id(line: &str, re: &Regex) -> u32 {
    re.captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("Can't read file");

    // Seems like we can't sort on an iterator, so we must turn it into a Vec?
    // `sort_by` mutates the Vec, hence the `mut`.
    let mut input: Vec<_> = input.lines().collect();
    dbg!(&input);
    input.sort_by(|a, b| parse_date(&a).timestamp().cmp(&parse_date(&b).timestamp()));

    let re_begin_shift = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let re_falls_asleep = Regex::new(r"falls asleep").unwrap();
    let re_wakes_up = Regex::new(r"wakes up").unwrap();

    let mut cur_guard: u32 = 0;
    let mut sleep_date = NaiveDateTime::from_timestamp(0, 0);
    let mut guard_sleep: HashMap<u32, i64> = HashMap::new();

    for line in input {
        if re_begin_shift.is_match(line) {
            cur_guard = parse_guard_id(&line, &re_begin_shift);
        } else if re_falls_asleep.is_match(line) {
            let date = parse_date(line);
            sleep_date = date;
        } else if re_wakes_up.is_match(line) {
            let date = parse_date(line);
            // This is quite naive, but we know that the guards only
            // sleep for X amount exact minutes.
            let minutes = guard_sleep.entry(cur_guard).or_insert(0);
            let mins = (date.timestamp() - sleep_date.timestamp()) / 60;
            *minutes += mins;
        }
    }

    dbg!(&guard_sleep);

    let max = guard_sleep
        .iter()
        .max_by(|(_, min), (_, other)| min.cmp(other))
        .unwrap();
    let (id, min) = max;
    dbg!(i64::from(*id) * min);
}
