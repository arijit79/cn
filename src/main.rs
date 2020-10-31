mod checks;
mod utils;

use clap::{App, Arg};
use std::path::PathBuf;
use utils::*;
use std::process::exit;

pub struct Abort;

pub fn copy_item(sources: Vec<String>, dest: PathBuf) {
    if dest.is_dir() {
        for i in sources {
            let mut d = PathBuf::from(&dest);
            d.push(PathBuf::from(&i).file_name().unwrap().to_str().unwrap());

            if PathBuf::from(&i).is_dir() {
                copy_dir(&i, PathBuf::from(d));
            } else {
                copy_file(&i, PathBuf::from(d));
            }
        }} else {
        for file in sources {
            copy_file(&file, dest.clone());
        }
    }
}

fn main() {
    let cli = App::new("cn")
        .version("0.1.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .short("r")
                .help("Recursively copy directories. Turned on by default")
        )
        .arg(
            Arg::with_name("Recursive")
                .short("R")
                .help("Alias to -r or --recursive")
        )
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

    let sources: Vec<&str> = cli.values_of("source").unwrap().collect();
    let mut owned_sources: Vec<String> = Vec::new();
    for item in sources {
        owned_sources.push(item.to_string());
    }
    let sources = owned_sources;
    let dest = PathBuf::from(cli.value_of("dest").unwrap());

    if !dest.exists() {
        senderr(format!("'{}' No such file or directory", dest.display()));
        exit(1);
    }

    if dest.metadata().unwrap().permissions().readonly() {
        senderr(format!("'{}' Permission denied", dest.display()));
        exit(1);
    }

    copy_item(sources, dest);
}
