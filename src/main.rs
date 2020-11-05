mod checks;
mod utils;
mod args;

use std::fs::remove_file;
use std::path::PathBuf;
use std::process::exit;
use utils::*;
pub use args::Flags;

// A struct to tell that furthur execution of the code should stop
pub struct Abort;

pub fn copy_item(sources: Vec<String>, dest: PathBuf, flags: Flags) {
    if dest.is_dir() {
        for i in sources {
            let source = PathBuf::from(&i);
            // Make a copy of dest, specifically for this iteration
            // and push the filename of the source item
            let mut d = PathBuf::from(&dest);
            d.push(source.file_name().unwrap());

            // If the source is a directory, call the copy_dir function else call the copy_file option
            if source.is_dir() {
                copy_dir(&i, d, &flags);
            } else {
                let copy_result = copy_file(&i, d, &flags);
                if copy_result.is_ok() && !flags.copy {
                    let _ = remove_file(&i);
                }
            }
        }
    } else {
        // If the destination is not a directory, coy the single file
        for file in sources {
            if PathBuf::from(&file).is_dir() && !dest.exists() {
                copy_dir(&file, dest.clone(), &flags)
            } else {
                let r = copy_file(&file, dest.clone(), &flags);
                if r.is_ok() && !flags.copy {
                    let move_result = remove_file(&file);
                    if move_result.is_err() {
                        move_result.unwrap_or_else(|why| {
                            senderr(format!("`{}`: Could not be deleted\n\n{}", file, why));
                        })
                    }
                }
            }
        }
    }
}

fn main() {
    let cli = args::matches();
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
            }
            Ok(_) => {}
            Err(_) => {
                senderr(format!("'{}' Could not get metadata", dest.display()));
                exit(1);
            }
        }
    }
    let flags = Flags::set(&cli);
    // Copy the sources
    copy_item(sources, dest, flags);
}
