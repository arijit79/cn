use crate::checks;
use crate::copy_item;
use ansi_term::Colour::Red;
use std::fs::{copy, create_dir};
use std::path::PathBuf;
use atty::Stream;

pub fn copy_dir(dir: &str, dest: PathBuf, copy: bool) {
    // Create the directory. We don't need to check for write
    // permission since we already check that before
    let _ = create_dir(&dest);
    // Initialize an empty vector to store subdirectories
    let mut dirs: Vec<String> = Vec::new();
    // Go through the items in the folder
    for item in std::fs::read_dir(&dir).unwrap() {
        // Get the item name and convert it into &str from OsStr
        let os_item_name = item.unwrap().file_name();
        let item_name = os_item_name.to_str().unwrap().to_string();

        // Keep a clone of the source specifically for this
        // iteration. Also push the item name
        let mut spath = PathBuf::from(dir);
        spath.push(&item_name);
        // If it is a subdirectory, keep it aside in the dirs vector
        if PathBuf::from(&spath).is_dir() {
            dirs.push(spath.to_str().unwrap().to_string());
        } else {
            // Keep a clone of the destination specifically for this
            // iteration. Also push the item name
            let mut dpath = dest.clone();
            dpath.push(&item_name);
            // Let's copy the file
            copy_file(&spath.to_str().unwrap(), dpath);
        }
    }
    // Finally, if there are subdirectories, go and copy them
    if !dirs.is_empty() {
        copy_item(dirs, dest, copy);
    }
}

pub fn copy_file(file: &str, dest: PathBuf) {
    // Generate the PathBuf for the file and do the checks
    let fp = PathBuf::from(file);
    let check_result = checks::check_all(&fp, &dest);

    // If there are any errors, immidiately abort
    if check_result.is_err() {
        return;
    }
    // Let's copy the file
    let copy_result = copy(fp.to_str().unwrap(), dest.to_str().unwrap());
    // Handle the error
    copy_result.unwrap_or_else(|r| {
        senderr(format!("An error while copying...\n{:?}", r.kind()));
        0
    });
}

pub fn senderr(e: String) {
    // Print things in the stderr
    if atty::is(Stream::Stderr) {
        eprintln!("{}", Red.bold().paint(e));
    } else {
        eprintln!("{}", e);
    }
}