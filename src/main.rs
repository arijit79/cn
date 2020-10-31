mod checks;

use ansi_term::Colour::Red;
use clap::{App, Arg};
use std::fs::copy;
use std::path::{Path, PathBuf};
use std::process::exit;

pub struct Abort;

pub fn senderr(e: String) {
    eprintln!("{}", Red.bold().paint(e));
}

pub fn copy_file(file: &str, mut dest: PathBuf) {
    let fp = PathBuf::from(file);
    let check_result = checks::check_all(&fp, &dest);

    if check_result.is_err() {
        return ();
    }

    let copy_result = copy(fp.to_str().unwrap(), dest.to_str().unwrap());

    if copy_result.is_err() {
        copy_result.unwrap_or_else(|why|{
            println!("{}", why);
            1
    });
        senderr(format!("'{}' Permission denied", dest.display()));
        exit(1);
    }
}

fn main() {
    let cli = App::new("cn")
        .version("0.1.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("source")
                .takes_value(true)
                .value_name("SOURCE")
                .multiple(true)
                .help("The paths that needs to be copied")
                .required(true),
        )
        .arg(
            Arg::with_name("dest")
                .takes_value(true)
                .value_name("DESTINATION")
                .help("The directory where the files need to be placed")
                .required(true),
        )
        .get_matches();

    let sources = cli.values_of("source").unwrap();
    let mut dest = PathBuf::from(cli.value_of("dest").unwrap());

    if !dest.exists() {
        senderr(format!("'{}' No such file or directory", dest.display()));
        exit(1);
    }

    let mut owned_sources: Vec<String> = Vec::new();

    if dest.is_dir() {
        for i in sources {
            let mut string = String::from(dest.to_str().unwrap());
            string.push_str(PathBuf::from(i).file_name().unwrap().to_str().unwrap());
            copy_file(i, PathBuf::from(string));
        }} else {
        for file in sources {
            copy_file(file, dest.clone());
        }
    }
}
