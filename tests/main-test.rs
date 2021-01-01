use std::io::{self, Write};
use std::path::{Path};
use std::process::Command;

/*
fn default_cal_stub() {
    // TODO: Write a calendar file containing a year's worth of calendar events,
    // starting from today.
}
*/

#[test]
fn default_ouput_matches_calendar() {
    let tests_dir_path = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
    let tests_dir_path_str = tests_dir_path.to_str().unwrap();
    let cal_path_str = format!("{}/calendars/calendar.backcompat", &tests_dir_path_str);

    let scmd = Command::new("cargo").args(&["run", "--", "-f", &cal_path_str]).output().expect("Failed to execute 'scalendar'");
    let ccmd = Command::new("calendar").args(&["-f", &cal_path_str]).output().expect("Failed to execute 'calendar'");
    assert_eq!(scmd.stdout, ccmd.stdout);
}

// TODO: Test behaviour of malformed lines
