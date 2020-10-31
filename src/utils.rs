use ansi_term::Colour::Red;
use std::path::PathBuf;
use std::fs::{copy, create_dir};
use crate::checks;
use crate::copy_item;

pub fn copy_dir(dir: &str, dest: PathBuf) {
    create_dir(&dest);
    let dir_path = PathBuf::from(dir);
    let mut dirs: Vec<String> = Vec::new();
    for item in std::fs::read_dir(&dir_path).unwrap() {
        let os_filename = item.unwrap().file_name();
        let filename = os_filename.to_str().unwrap().to_string();

        let mut spath = dir_path.clone();
        spath.push(&filename);
        if PathBuf::from(&spath).is_dir() {
            dirs.push(spath.to_str().unwrap().to_string());
        } else {
            let mut dpath = dest.clone();
            dpath.push(&filename);
            copy_file(&spath.to_str().unwrap(), dpath);
        }
    }
    if ! dirs.is_empty() {
        copy_item(dirs, dest);
    }
}

pub fn copy_file(file: &str, dest: PathBuf) {
    let fp = PathBuf::from(file);
    let check_result = checks::check_all(&fp);

    if check_result.is_err() {
        return ();
    }

    let copy_result = copy(fp.to_str().unwrap(), dest.to_str().unwrap());

    copy_result.unwrap_or_else(|r| {
        senderr(format!("An error while copying...\n{:?}", r.kind()));
        0
    });
}

pub fn senderr(e: String) {
    eprintln!("{}", Red.bold().paint(e));
}