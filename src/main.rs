mod checks;
mod utils;

use clap::{App, Arg};
use std::path::PathBuf;
use std::process::exit;
use utils::*;

// A struct to tell that furthur execution of the code should stop
pub struct Abort;

pub fn copy_item(sources: Vec<String>, dest: PathBuf) {
    if dest.is_dir() {
        for i in sources {
            let source = PathBuf::from(&i);
            // Make a copy of dest, specifically for this iteration
            // and push the filename of the source item
            let mut d = PathBuf::from(&dest);
            d.push(source.file_name().unwrap().to_str().unwrap());

            // If the source is a directory, call the copy_dir function else call the copy_file option
            if source.is_dir() {
                copy_dir(&i, PathBuf::from(d));
            } else {
                copy_file(&i, PathBuf::from(d));
            }
        }
    } else {
        // If the destination is not a directory, coy the single file
        for file in sources {
            copy_file(&file, dest.clone());
        }
    }
}

fn main() {
    // Get the command-line matches
    let cli = App::new("cn")
        .version("0.1.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .short("r")
                .help("Recursively copy directories. Turned on by default"),
        )
        .arg(
            Arg::with_name("Recursive")
                .short("R")
                .help("Alias to -r or --recursive"),
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

    // Get the sources in Vec<&str> and convert them to Vec<String>
    let sources: Vec<&str> = cli.values_of("source").unwrap().collect();
    let mut owned_sources: Vec<String> = Vec::new();
    for item in sources {
        owned_sources.push(item.to_string());
    }
    let sources = owned_sources;

    // Get the destination and check if it exists
    let dest = PathBuf::from(cli.value_of("dest").unwrap());
    if !dest.exists() {
        senderr(format!("'{}' No such file or directory", dest.display()));
        exit(1);
    }

    // Check if we have write permissions
    if dest.metadata().unwrap().permissions().readonly() {
        senderr(format!("'{}' Permission denied", dest.display()));
        exit(1);
    }

    // Copy the sources
    copy_item(sources, dest);
}
