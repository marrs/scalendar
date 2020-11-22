// Defintions
// ==========
//
// Range = number of days to display

use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    let cmd = Command::new("calendar").arg("-l365").output().expect("Failed to execute 'calendar -l365'");

    // TODO:
    //   * Show events for current range:
    //     * value passed to -l,
    //     * 1 by default.
    //   * Include events with reminder that falls within range.
    io::stdout().write_all(&cmd.stdout).unwrap();
    io::stderr().write_all(&cmd.stderr).unwrap();

/*
assert!(output.status.success());
    String::from_utf8(lsCmd.stdout)
        .lines()
        .for_each(|x| println!("{:?}", x));
*/
}
