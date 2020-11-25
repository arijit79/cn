//Declare all modules
mod args;
mod checks;
mod copy;
// TODO: RE-implement when clap::IntoApp is complete
// mod gen_completions;
mod hard_links;
mod soft_links;
mod utils;

use args::{Cli, Flags};
use clap::Clap;
use copy::copy_item;
use std::process::exit;

// Exit codes
#[allow(dead_code)]
const STATUS_OK: i32 = 0;
const STATUS_ERR: i32 = 1;

// A signal to stop furthur execution of the program
pub struct Abort;

#[async_std::main]
async fn main() {
    // Get all CLI arguments
    let cli = Cli::parse();

    // // TODO: RE-implement when clap::IntoApp is complete
    // If completion is present, generate it and exit
    // if let Some(ref scmd) = cli.subcmd {
    //     if let args::SubCommand::Completion(c) = scmd {
    //         gen_completions::generate_completions(cli);
    //         exit(STATUS_OK);
    //     }
    // }

    // Initialize the flags
    let flags = Flags::set(&cli);

    let mut sources = cli.paths;
    // Get destination in PathBuf
    let dest = if cli.target_directory.is_some() {
        cli.target_directory.unwrap()
    } else {
        if sources.len() > 1 {
            sources.pop().unwrap()
        } else {
            utils::senderr("PATHS must have at least two arguments unless -t is given");
            exit(STATUS_ERR);
        }
    };
    // If -m flag is given, send a warning
    if cli.r#move {
        utils::move_warning();
    }
    // Start copying/linking the various sources
    if cli.hard_link {
        hard_links::hl_item(sources, dest, &flags).await;
    } else if cli.symbolic_links {
        #[cfg(target_family = "unix")]
        soft_links::unix_symlink::sl_item(sources.clone(), dest, &flags).await;
        #[cfg(target_family = "windows")]
        soft_links::win_symlink::sl_item(sources.clone(), dest, &flags).await;
    } else {
        copy_item(sources, dest.clone(), &flags).await;
    }
}
