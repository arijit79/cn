//Declare all modules
mod args;
mod checks;
mod copy;
mod gen_completions;
mod hard_links;
mod soft_links;
mod utils;

use args::{cli, Flags};
use async_std::path::PathBuf;
use copy::copy_item;
use std::process::exit;

// Exit codes
const STATUS_OK: i32 = 0;
const STATUS_ERR: i32 = 1;

// A signal to stop furthur execution of the program
pub struct Abort;

#[async_std::main]
async fn main() {
    // Get all CLI arguments
    let app = cli();
    // Kepp a clone of app, for use in gen_completion function, since `get_matches()` will consume it
    let app_clone = app.clone();
    let matches = app.get_matches();

    // If completion is present, generate it and exit
    if matches.is_present("completion") {
        gen_completions::generate_completions(app_clone);
        exit(STATUS_OK);
    }

    // Get all sources in PathBuf
    if !matches.is_present("paths") {
        utils::senderr("At least one source must be given");
        exit(STATUS_ERR);
    }
    let mut sources: Vec<PathBuf> = Vec::new();
    matches
        .values_of("paths")
        .unwrap()
        .for_each(|i| sources.push(PathBuf::from(i)));
    // Get destination in PathBuf
    let dest = if matches.is_present("target-directory") {
        PathBuf::from(matches.value_of("target-directory").unwrap())
    } else {
        if sources.len() > 1 {
            sources.pop().unwrap()
        } else {
            utils::senderr("PATHS must have at least two arguments unless -t is given");
            exit(STATUS_ERR);
        }
    };
    // Initialize the flags
    let flags = Flags::set(&matches);
    // If -m flag is given, send a warning
    if !flags.copy {
        utils::move_warning();
    }
    // Start copying/linking the various sources
    if matches.is_present("hard-link") {
        hard_links::hl_item(sources, dest, &flags).await;
    } else if matches.is_present("symbolic-link") {
        #[cfg(target_family = "unix")]
        soft_links::unix_symlink::sl_item(sources.clone(), dest, &flags).await;
        #[cfg(target_family = "windows")]
        soft_links::win_symlink::sl_item(sources.clone(), dest, &flags).await;
    } else {
        copy_item(sources, dest.clone(), &flags).await;
    }
}
