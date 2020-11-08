use ansi_term::Colour::Red;
use async_std::path::PathBuf;
use atty::Stream;
use std::io::{prelude::*, stdin, stdout};

pub fn senderr(e: String) {
    // Print things in the stderr
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Red.bold().paint(e));
    } else {
        eprintln!("{}", e);
    }
}

pub fn prompt(path: &PathBuf) -> bool {
    print!(
        "'{}': Already present in destination. Overwrite [y/n] ",
        path.display()
    );
    let _ = stdout().flush();
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer == "y"
}
