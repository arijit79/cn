use crate::{checks, Abort, Flags};
use ansi_term::Colour::Red;
use atty::Stream;
use std::fs::remove_dir_all;
use std::fs::{copy, create_dir};
use std::path::PathBuf;
use std::io::{stdin, stdout, prelude::*};

fn prompt(path: &PathBuf) -> bool {
    print!("'{}': Already present in destination. Overwrite [y/n] ", path.display());
    let _ = stdout().flush();
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer == "y"
}

pub fn copy_dir(dir: &str, dest: PathBuf, flags: &Flags) {
    // We have got no errors
    let mut errors = false;
    // Create the directory. We don't need to check for write
    if flags.interactive && dest.exists() {
        if prompt(&dest) {
            let _ = create_dir(&dest);
        }
    } else {
        let _ = create_dir(&dest);
    }
    // Print output when vrbose flag is on
    if flags.verbose {
        println!("{} -> {}", dir, dest.display());
    }
    
    // Go through the items in the folder
    for item in std::fs::read_dir(&dir).unwrap() {
        // Get the item name and convert it into &str from OsStr
        let os_item_name = item.unwrap().file_name();
        let item_name = os_item_name.to_str().unwrap().to_string();

        // Keep a clone of the source specifically for this
        // iteration. Also push the item name
        let mut spath = PathBuf::from(dir);
        spath.push(&item_name);

        // Keep a clone of the destination specifically for this
        // iteration. Also push the item name
        let mut dpath = dest.clone();
        dpath.push(&item_name);
        // If it is a subdirectory, copy it too
        if spath.is_dir() {
            copy_dir(spath.to_str().unwrap(), dpath, &flags);
        } else {
            // Let's copy the file
            let r = copy_file(&spath.to_str().unwrap(), dpath, &flags);
            if r.is_err() {
                errors = true;
            }
        }
    }
    if !errors && !flags.copy {
        let _ = remove_dir_all(&dir);
    }
}

pub fn copy_file(file: &str, dest: PathBuf, flags: &Flags) -> Result<(), Abort> {
    // Generate the PathBuf for the file and do the checks
    let fp = PathBuf::from(file);
    let check_result = checks::check_all(&fp, &dest);

    // If there are any errors, immidiately abort
    if check_result.is_err() {
        return Err(Abort);
    }
    // Let's copy the file
    if flags.interactive && dest.exists() {
        if prompt(&dest) {
            let copy_result = copy(fp.to_str().unwrap(), dest.to_str().unwrap());
            // Handle the error
            if copy_result.is_err() {
                copy_result.unwrap_or_else(|r| {
                    senderr(format!("An error while copying...\n{:?}", r.kind()));
                    0
                });
                return Err(Abort);
            }
        }
    } else {
        let copy_result = copy(fp.to_str().unwrap(), dest.to_str().unwrap());
        // Handle the error
        if copy_result.is_err() {
            copy_result.unwrap_or_else(|r| {
                senderr(format!("An error while copying...\n{:?}", r.kind()));
                0
            });
            return Err(Abort);
        }
    }
    // Print output when vrbose flag is on
    if flags.verbose {
        println!("{} -> {}", fp.display(), dest.display());
    }
    Ok(())
}

pub fn senderr(e: String) {
    // Print things in the stderr
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Red.bold().paint(e));
    } else {
        eprintln!("{}", e);
    }
}
