//Declare all modules
mod args;
mod checks;
mod functions;
pub mod utils;

use args::{matches, Flags};
use async_std::path::PathBuf;
use functions::{copy_dir, copy_file};
use futures::future::join_all;

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
    // Start copying the various sources
    copy_item(sources, dest.clone(), &flags).await;
}

pub async fn copy_item(sources: Vec<PathBuf>, dest: PathBuf, flags: &Flags) {
    // If the dest is a file, only copy the first source and leave the rest
    if dest.is_file().await {
        copy_file(sources[0].clone(), dest, flags).await;
    } else {
        // Make a Vec of all tasks
        let mut tasks = vec![];
        for i in sources {
            // Push appropriate function for the itemtype
            if i.is_dir().await {
                tasks.push(copy_dir(i.clone(), dest.clone(), flags));
            } else {
                tasks.push(Box::pin(copy_file(i.clone(), dest.clone(), &flags)));
            }
        }
        // Run all the tasks
        join_all(tasks).await;
    }
}
