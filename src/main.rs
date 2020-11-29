// Defintions
// ==========
//
// days = number of days to display

use std::process::Command;
use std::env;
use std::io::{self, Write};
use regex::Regex;

type Days = u32;

struct Args {
    // TODO: Implement error message
    days_to_come: Days,
    days_past: Days,
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
    let mut days_to_come = 1;
    let mut days_past = 0;
    let mut calendar_file = None;
    for arg in args {
        // TODO: Implement -B
        if found_flag == "-l" || found_flag == "-A" {
            if pos_int_regex.is_match(&arg) {
                //log_info("Future days argument value found");
                days_to_come = arg.parse::<u32>().unwrap();
            } else {
                //log_error("Future days argument value malformed");
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
    return Args {
        days_to_come: days_to_come,
        days_past: days_past,
        calendar_file: calendar_file
    };
}

fn main() {
    // TODO:
    //   * Show events for current range:
    //     * value passed to -l,
    //     * 1 by default.
    //   * Include events with reminder that falls within Future days.
    let args = parse_args(env::args());
    let cmd = Command::new("calendar").arg("-l365").args(args.to_vec(&["-f"])).output().expect("Failed to execute 'calendar -l365'");

    io::stdout().write_all(&cmd.stdout).unwrap();
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
