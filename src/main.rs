// Defintions
// ==========
//
// days = number of days to display

use std::process::Command;
use std::env;
use std::io::{self, Write};
use regex::Regex;
use chrono::{DateTime, NaiveDate, Datelike, Utc};

type Days = u32;

struct Args {
    // TODO: Implement error message
    days_to_come: Option<Days>,
    days_past: Option<Days>,
    calendar_file: Option<String>
}

impl Args {
    fn to_vec(&self, flags: &[&str]) -> Vec<String> {
        let mut result = vec!();
        for x in flags.iter() {
            match x {
                &"-f" => {
                    match &self.calendar_file {
                        Some(xx) => {
                            result.push("-f".to_string());
                            result.push(xx.to_string());
                        },
                        None     => ()
                    }
                },
                _ => () // TODO: Log error

            }
        }
        return result;
    }
}

enum LogDest {
    None,
    StdErr,
}

static mut LOG_DEST: LogDest = LogDest::None;

/*
fn log_info(x: &str) {
    match LOG_DEST {
        LogDest::None => (),
        LogDest::StdErr => io::stderr().write_all(&x[..]).unwrap()
    }
}

fn log_error(x: &str) {
    match LOG_DEST {
        LogDest::None => (),
        LogDest::StdErr => io::stderr().write_all(x).unwrap()
    }
}
*/

fn parse_args(args: env::Args) -> Args {
    let pos_int_regex = Regex::new(r"^\d+$").unwrap();
    let mut found_flag = "";
    let mut days_to_come: Option<Days> = None;
    let mut days_past: Option<Days> = None;
    let mut calendar_file = None;
    for arg in args {
        if found_flag == "-l" || found_flag == "-A" {
            if pos_int_regex.is_match(&arg) {
                //log_info("Future days argument value found");
                days_to_come = Some(arg.parse::<u32>().unwrap());
            } else {
                //log_error("Future days argument value malformed");
            }
            found_flag = "";
        } else if found_flag == "-B" {
            if pos_int_regex.is_match(&arg) {
                //log_info("Past days argument value found");
                days_past = Some(arg.parse::<u32>().unwrap());
            } else {
                //log_error("Past days argument value malformed");
            }
            found_flag = "";
        } else if found_flag == "-f" {
            calendar_file = Some(arg);
            found_flag = "";
        } else if arg == "-l" || arg == "-A" {
            //log_info("Future days argument found");
            found_flag = "-l";
        } else if arg == "-f" {
            found_flag = "-f";
        } else if arg == "-v" {
            unsafe {
                LOG_DEST = LogDest::StdErr
            }
        }
    }
    // Check for conflicting args
    if days_to_come.is_some() && days_past.is_some() {
        panic!("-A (or -l) and -B are mutually exclusive options.");
    }
    // Set default args
    if days_to_come.is_none() && days_past.is_none() {
        days_to_come = Some(1);
    }
    return Args {
        days_to_come: days_to_come,
        days_past: days_past,
        calendar_file: calendar_file
    };
}

struct Event<'a> {
    ordinal: u32,
    offset: i32,
    date: &'a str,
    desc: String,
}

fn main() {
    // TODO:
    //   [ ] Show events for current range:
    //       * value passed to -l,
    //       * 1 by default.
    //       [*] Identify date part of calendar lines
    //       [*] Parse date part
    //       [.] Convert dates to days from today
    //         [*] For -A
    //         [ ] For -B
    //       [ ] Show those days that fall within range
    //   * Include events with reminder that falls within Future days.
    let args = parse_args(env::args());
    let cmd = Command::new("calendar").arg("-l365").args(args.to_vec(&["-f"])).output().expect("Failed to execute 'calendar -l365'");

    let cal_data = String::from_utf8(cmd.stdout).unwrap();
    let cal_lines = cal_data.split("\n");
    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month();
    let days_in_current_year = NaiveDate::parse_from_str(&format!("{} {}", current_year, "Dec 31"), "%Y %b %d").unwrap().ordinal();
    let now_ordinal_i32 = now.ordinal() as i32;
    let mut events: Vec<Event> = vec![];
    for line in cal_lines {
        if line == "" {
            continue;
        }
        let first_char = line.chars().next().unwrap();
        if first_char == '\t' || first_char == ' ' {
            //TODO: Get multiline descriptions for events
            //events[events.len() -1].desc = format!("{}\n{}", &events[events.len() -1].desc, line);
        } else {
            let mut date = NaiveDate::parse_from_str(&format!("{} {}", current_year, &line[0..6])[..], "%Y %b %d");
            if date.unwrap().month() < current_month
            && args.days_to_come.is_some() {
                date = NaiveDate::parse_from_str(&format!("{} {}", current_year +1, &line[0..6])[..], "%Y %b %d");
            } else if date.unwrap().month() > current_month
              && args.days_past.is_some() {
                date = NaiveDate::parse_from_str(&format!("{} {}", current_year -1, &line[0..6])[..], "%Y %b %d");
            }
            match date {
                Ok(date) => {
                    let date_ordinal = date.ordinal();
                    let date_ordinal_i32 = date_ordinal as i32;
                    events.push(Event {
                        ordinal: date_ordinal,
                        offset: if args.days_to_come.is_some() {
                            if date_ordinal_i32 < now_ordinal_i32 {
                                (now_ordinal_i32 + (days_in_current_year as i32 - now_ordinal_i32) + date_ordinal_i32) - now_ordinal_i32
                            } else {
                                date_ordinal_i32 - now_ordinal_i32
                            }
                        } else {
                            now_ordinal_i32 - date_ordinal_i32
                        },
                        date: &line[0..6],
                        desc: line[6..].to_string()
                    });
                },
                Err(err) => println!("{}", err)
            }
        }
    }
    for event in events {
        println!("{} {} {}", event.offset, event.date, event.desc);
    }
    //io::stdout().write_all(cal_data.as_bytes()).unwrap();
    io::stderr().write_all(&cmd.stderr).unwrap();

    //log_info(&format!("days to come: {}", args.days_to_come)[..]);
    //log_info(&format!("days past: {}", args.days_past)[..]);

/*
assert!(output.status.success());
    String::from_utf8(lsCmd.stdout)
        .lines()
        .for_each(|x| println!("{:?}", x));
*/
}
