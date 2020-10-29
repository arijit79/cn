use crate::senderr;
use std::fs::File;
use std::path::PathBuf;

/* REFERENCE FOR ERROR CODES
0 - Successful
1 - Source or destination does not exists
2 - Permission error
*/

pub fn check_all(s: &PathBuf, d: &PathBuf) {
    // Check if source exists and it can be read
    if !s.exists() {
        senderr(format!("'{}' No such file or directory", s.display()), 1)
    }
    if File::open(&s).is_err() {
        senderr(format!("'{}' Permission denied", s.display()), 2)
    }

    // Check if the dest folder exists
    if !d.exists() {
        senderr(format!("'{}' No such file or directory", d.display()), 3)
    }
}
