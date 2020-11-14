//Declare all modules
mod args;
mod checks;
mod copy;
mod hard_links;
mod soft_links;
mod utils;

use args::{matches, Flags};
use async_std::path::PathBuf;
use copy::copy_item;
use soft_links::unix_symlink::sl_item;

// A signal to stop furthur execution of the program
pub struct Abort;

#[async_std::main]
async fn main() {
    // Get all CLI arguments
    let cli = matches();
    // Get all sources in PathBuf
    let mut sources: Vec<PathBuf> = Vec::new();
    cli.values_of("source")
        .unwrap()
        .for_each(|i| sources.push(PathBuf::from(i)));
    // Get destination in PathBuf
    let dest = PathBuf::from(cli.value_of("dest").unwrap());
    // Initialize the flags
    let flags = Flags::set(&cli);
    if !flags.copy {
        utils::move_warning();
    }
    // Start copying/linking the various sources
    if cli.is_present("hard-link") {
        hard_links::hl_item(sources, dest, &flags).await;
    } else if cli.is_present("symbolic-link") {
        sl_item(sources.clone(), dest, &flags).await;
    } else {
        copy_item(sources, dest.clone(), &flags).await;
    }
}
