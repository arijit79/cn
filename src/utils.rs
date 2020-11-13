use crate::Abort;
use ansi_term::Colour::Red;
use async_std::path::PathBuf;
use atty::Stream;
use std::io::Error;
use std::io::{prelude::*, stdin, stdout};

pub fn senderr<E>(e: E)
where
    E: Into<String>,
{
    let output = String::from(e.into());
    // Print things in the stderr
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Red.bold().paint(output));
    } else {
        eprintln!("{}", output);
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

pub fn check_err<M, T>(message: M, r: Result<T, Error>, path: Option<&PathBuf>) -> Result<(), Abort>
where
    M: std::fmt::Display,
{
    if r.is_err() {
        let p;
        if path.is_none() {
            p = "".to_string()
        } else {
            p = format!("{:?}", path.unwrap().as_os_str());
        }
        if let Err(e) = r {
            let output = format!("{}{}\n{:?}", message, &p, e.kind());
            senderr(output);
        }
        return Err(Abort);
    }
    Ok(())
}
