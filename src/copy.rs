use crate::args::Flags;
use crate::checks::check_all;
use crate::utils::{check_err, prompt};
use async_std::fs::{copy, create_dir, read_dir, remove_dir_all, remove_file};
use async_std::path::PathBuf;
use async_std::prelude::*;
use futures::future::join_all;

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

#[async_recursion::async_recursion]
pub async fn copy_dir(s: PathBuf, dest: PathBuf, flags: &Flags) {
    // Get a clone of the destination
    let mut d = dest.clone();
    // If `d` is a directory, push the filename after it
    // so that the directory is created inside it
    if d.is_dir().await {
        d.push(&s.file_name().unwrap());
    }
    // If interactive and `d` exists, go for prompting
    if flags.interactive && d.exists().await {
        if prompt(&d) {
            // Remove here, otherwise create_dir will fail
            let _ = remove_dir_all(&d).await;
            if create_dir(&d).await.is_ok() && flags.verbose {
                println!("{} -> {}", s.display(), d.display())
            }
        }
    } else {
        // Go ahead here, do not call remove_dir_all since it may
        // fail, because the directory may not exists
        // (We check for both things at once)
        if create_dir(&d).await.is_ok() && flags.verbose {
            println!("{} -> {}", s.display(), d.display())
        }
    }
    // Read the source directory and init a tasks vec
    let mut dir_read = read_dir(&s).await.unwrap();
    let mut tasks = vec![];
    while let Some(item) = dir_read.next().await {
        // Read the item and generate the destination path
        let mut name = PathBuf::from(&s);
        name.push(item.unwrap().file_name());
        // If it is a directory, push a async recursion to tasks
        if name.is_dir().await {
            tasks.push(copy_dir(name, d.clone(), flags));
        } else {
            // Else push a copy_file in tasks. Make sure to pin it
            tasks.push(Box::pin(copy_file(name, d.clone(), flags)))
        }
    }
    // Wait for all jobs to finish
    join_all(tasks).await;
    // Check if it is a move operation
    if !flags.copy {
        // If it were a move operation, there would be no files left
        // in source
        let mut entries = read_dir(&s).await.unwrap();
        if entries.next().await.is_none() {
            let _ = remove_dir_all(s).await;
        }
    }
}

pub async fn copy_file(s: PathBuf, d: PathBuf, flags: &Flags) {
    // Get a copy of cdest and push the filename
    let mut d = d.clone();
    // if destination is a directory, push the filename inside `d`
    if d.is_dir().await {
        d.push(&s.file_name().unwrap());
    }
    // Check if the file passes all checks
    let checks = check_all(&s, &d).await;
    if checks.is_err() {
        return;
    }
    let async_copy = async {
        let result = copy(&s, &d).await;
        // If there are no errors, remove the source ifit was a
        // move operation
        if !check_err("There was an error copying", result, Some(&s)).is_err() {
            // If there are no errors, remove the source if it
            // was a   move operation
            if !flags.copy {
                let _ = remove_file(&s).await;
            }
            // If verbose mode, print some things
            if flags.verbose {
                println!("{} -> {}", s.display(), d.display())
            }
        }
    };
    // If it is interactive and d exists, prompt the user
    if flags.interactive && d.exists().await {
        if prompt(&d) {
            async_copy.await;
        }
    } else {
        // If it was a non-interactive operation, do the same things as above
        async_copy.await;
    }
}
