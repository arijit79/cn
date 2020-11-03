mod checks;
mod utils;

use clap::{App, Arg};
use std::path::PathBuf;
use std::process::exit;
use utils::*;
use std::fs::{remove_dir_all,remove_file};

// A struct to tell that furthur execution of the code should stop
pub struct Abort;

pub fn copy_item(sources: Vec<String>, dest: PathBuf, copy: bool) {
    if dest.is_dir() {
        for i in sources {
            let source = PathBuf::from(&i);
            // Make a copy of dest, specifically for this iteration
            // and push the filename of the source item
            let mut d = PathBuf::from(&dest);
            d.push(source.file_name().unwrap().to_str().unwrap());

            // If the source is a directory, call the copy_dir function else call the copy_file option
            if source.is_dir() {
                copy_dir(&i, d, copy);
                if !copy {
                    let _ = remove_dir_all(&i);
                }
            } else {
                copy_file(&i, d);
                if !copy {
                    let _ = remove_file(&i);
                }
            }
        }
    } else {
        // If the destination is not a directory, coy the single file
        for file in sources {
            copy_file(&file, dest.clone());
            if !copy {
                let _ = remove_file(&file);
            }
        }
    }
}

fn main() {
    // Get the command-line matches
    let cli = App::new("cn")
        .version("1.0.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .short("r")
                .conflicts_with("Recursive")
                .help("Recursively copy directories. Turned on by default"),
        )
        .arg(
            Arg::with_name("move")
                .long("move")
                .short("m")
                .help("Move source to destination rather than copying them"),
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

    // Check if many sources are given to destination which is a file
    if sources.len() > 1 && (dest.is_file() || !dest.exists()) {
        senderr(format!(
            "'{}' Multiple sources given to a single source file",
            dest.display()
        ));
        exit(2);
    }

    // Check if we have write permissions
    if dest.exists() {
        let meta = dest.metadata();
        match meta {
            Ok(m) if m.permissions().readonly() => {
                senderr(format!("'{}' Permission denied", dest.display()));
                exit(1);
            },
            Ok(_) => {},
            Err(_) => {
                senderr(format!("'{}' Could not get metadata", dest.display()));
                exit(1);
            }
        }
    }

    // Copy the sources
    if cli.is_present("move") {
        copy_item(sources, dest, false);
    } else {
        copy_item(sources, dest, true);
    }
}
