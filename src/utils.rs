use crate::Abort;
use ansi_term::Colour::{Red, Yellow};
use async_std::path::PathBuf;
use atty::Stream;
use std::io::Error;
use std::io::{prelude::*, stdin, stdout};

pub fn senderr<E>(e: E)
where
    E: Into<String>,
{
    let output = String::from(e.into());
    // Print error with color in the stderr only if stderr is the console
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Red.bold().paint(output));
    } else {
        eprintln!("{}", output);
    }
}

// Prompt user if a path should be overwritten
pub fn prompt(path: &PathBuf) -> bool {
    print!(
        "'{}': Already present in destination. Overwrite [Yes/no] ",
        path.display()
    );
    let _ = stdout().flush();
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    const CORRECTS: [&str; 4] = ["y", "Y", "yes", "Yes"];
    CORRECTS.contains(&buffer.as_str())
}

// Check a Result and nicely display an error
pub fn check_err<M, T>(message: M, r: Result<T, Error>, path: Option<&PathBuf>) -> Result<(), Abort>
where
    M: std::fmt::Display,
{
    if r.is_err() {
        let p;
        // Set path to empty string if no path is given
        if path.is_none() {
            p = "".to_string()
        } else {
            p = format!("{:?}", path.unwrap().as_os_str());
        }
        // Write the error
        if let Err(e) = r {
            let output = format!("{}{}\n{:?}", message, &p, e.kind());
            senderr(output);
        }
        return Err(Abort);
    }
    Ok(())
}

// Display a warning if the move flag is used
// THIS SHOULD BE REMOVED ONCE THE MOVE FLAG IS COMPLETELY DEPRECATED
pub fn move_warning() {
    const MESSAGE: &str = "The -m or --move flag is soft-deprecated and use it's use is discouraged";
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Yellow.paint(MESSAGE))
    } else {
        eprintln!("{}", MESSAGE);
    }
}