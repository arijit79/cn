use crate::{senderr, Abort};
use std::fs::File;
use std::path::PathBuf;

/* REFERENCE FOR ERROR CODES
0 - Successful
1 - Permission error
*/

pub fn check_all(s: &PathBuf, d: &PathBuf) -> Result<(), Abort> {
    // Check if source exists and it can be read
    if !s.exists() {
        senderr(format!("'{}' No such file or directory", s.display()));
        return Err(Abort);
    }
    if File::open(&s).is_err() {
        senderr(format!("'{}' Permission denied", s.display()));
        return Err(Abort);
    }
    Ok(())
}
