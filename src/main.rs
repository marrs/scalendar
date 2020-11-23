// Defintions
// ==========
//
// days = number of days to display

use std::process::Command;
use std::env;
use std::io::{self, Write};
use regex::Regex;

struct Args {
    days_to_come: u32,
    days_past: u32,
}

fn parse_args(args: env::Args) -> Args {
    let pos_int_regex = Regex::new(r"^\d+$").unwrap();
    let mut found_flag = "";
    let mut parsed_args = Args { days_to_come: 1, days_past: 0 };
    for arg in args {
        // TODO: Implement -B
        if found_flag == "-l" || found_flag == "-A" {
            if pos_int_regex.is_match(&arg) {
                println!("Future days argument value found");
                parsed_args.days_to_come = arg.to_string().parse::<u32>().unwrap();
            } else {
                println!("Future days argument value malformed");
            }
            found_flag = "";
        } else if arg == "-l" || arg == "-A" {
            println!("Future days argument found");
            found_flag = "-l";
        }
    }
    return parsed_args;
}

fn main() {
    println!("Hello, world!");
    let cmd = Command::new("calendar").arg("-l365").output().expect("Failed to execute 'calendar -l365'");

    // TODO:
    //   * Show events for current range:
    //     * value passed to -l,
    //     * 1 by default.
    //   * Include events with reminder that falls within Future days.
    io::stdout().write_all(&cmd.stdout).unwrap();
    io::stderr().write_all(&cmd.stderr).unwrap();

    let args = parse_args(env::args());
    println!("days to come: {}", args.days_to_come);
    println!("days past: {}", args.days_past);

/*
assert!(output.status.success());
    String::from_utf8(lsCmd.stdout)
        .lines()
        .for_each(|x| println!("{:?}", x));
*/
}
