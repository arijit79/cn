use crate::{
    args::Flags,
    checks::check_all,
    utils::{prompt, senderr},
};
use async_std::fs::{hard_link, remove_file};
use async_std::path::PathBuf;
use std::process::exit;

pub async fn hl_item(sources: Vec<PathBuf>, mut dest: PathBuf, flags: &Flags) {
    // RESTRICTION: Only one can be hard linked at once
    if sources.len() > 1 {
        senderr(format!(
            "Only one source could be given when making hard links"
        ));
        exit(2);
    }
    let source = &sources[0];
    // Check if source is a regular file
    if !source.is_file().await {
        senderr("Source must be a regular file".to_string());
        exit(2);
    }
    // If destination is a directory, push the source filename to the dest
    if dest.is_dir().await {
        dest.push(source.file_name().unwrap());
    }

    // If dest exists, remove it
    if dest.exists().await && flags.interactive {
        if prompt(&dest) {
            let _ = remove_file(&dest).await;
        }
    } else {
        let _ = remove_file(&dest).await;
    }
    // Run checks
    let basic_checks = check_all(&source, &dest).await;
    if basic_checks.is_err() {
        exit(1);
    }
    // Make the link
    let _ = hard_link(source, dest).await;
}
