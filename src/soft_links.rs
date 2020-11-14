use crate::{
    args::Flags,
    checks::{check_all, check_canonical},
    utils::{check_err, prompt, senderr},
};
use async_std::prelude::*;
use async_std::{fs::create_dir, path::PathBuf};
use futures::future::join_all;
use std::process::exit;

// Only available for Unix(-like) platforms
// Windows support coming soon
#[cfg(target_family = "unix")]
pub mod unix_symlink {
    use super::*;
    use async_std::os::unix::fs::symlink;

    pub async fn sl_item(sources: Vec<PathBuf>, dest: PathBuf, flags: &Flags) {
        // If the destination is a file, immediately run `sl_file` and quit
        if dest.is_file().await {
            sl_file(sources[0].clone(), dest.clone(), flags).await;
        } else {
            // A vector of all tasks
            let mut tasks = vec![];
            for i in sources {
                if i.is_dir().await {
                    // Check if none of the paths are relative
                    if i.is_relative() || dest.is_relative() {
                        senderr("Relative paths are not allowed when symlinking directories");
                        exit(2);
                    }
                    // Add a task of symlinking the contents of directory
                    tasks.push(sl_dir(i, dest.clone(), flags));
                } else {
                    // If it is a file, check if the source and dest are in the same dir
                    if i.is_relative() && dest.is_relative() {
                        check_canonical(&i, &dest);
                    } else if i.is_relative() ^ dest.is_relative() {
                        // Error if there is a mix of relative and absolute paths
                        senderr("Absolute and relative paths cannot be used simultaniously");
                        exit(2);
                    }
                    // Add a task of symlinking a file
                    tasks.push(Box::pin(sl_file(i, dest.clone(), flags)));
                }
            }
            join_all(tasks).await;
        }
    }

    #[async_recursion::async_recursion]
    pub async fn sl_dir(s: PathBuf, mut d: PathBuf, flags: &Flags) {
        // Check for write permissions in the dest folder
        if d.parent()
            .unwrap()
            .metadata()
            .await
            .unwrap()
            .permissions()
            .readonly()
        {
            senderr(format!("`{}` Permission denied", d.display()));
            return;
        }
        // Push the source file name
        d.push(s.file_name().unwrap());

        // A function to Create the directory
        let init_dir = async {
            let r = create_dir(&d).await;
            if check_err("There was an error creating directory", r, Some(&d)).is_err() {
                return;
            } else {
                if flags.verbose {
                    println!("{} -> {}", s.display(), d.display())
                }
            }
        };

        // If the directory already exists and interactive mode is turned on, prompt 
        // for overwrite
        if d.exists().await {
            if flags.interactive {
                if prompt(&d) {
                    // if the user confirms, remove it and create the dir
                    let _ = async_std::fs::remove_dir_all(&d).await;
                    init_dir.await;
                }
            } else {
                // If no interactive, do the same as if it was confirmed
                let _ = async_std::fs::remove_dir_all(&d).await;
                init_dir.await;
            }
        } else {
            // Create dir if it dosen't exists
            init_dir.await;
        }
        // A Vec of all tasks
        let mut tasks = vec![];
        let mut entries = async_std::fs::read_dir(&s).await.unwrap();
        while let Some(p) = entries.next().await {
            // For each entry, add a task for it's respective function
            let name = p.unwrap().file_name();
            let mut source = s.clone();
            source.push(&name);
            if source.is_dir().await {
                tasks.push(sl_dir(source, d.clone(), flags));
            } else {
                tasks.push(Box::pin(sl_file(source, d.clone(), flags)));
            }
        }
        // Run all tasks
        join_all(tasks).await;
    }

    pub async fn sl_file(mut s: PathBuf, mut d: PathBuf, flags: &Flags) {
        // If dest is a dir, append the file name of the source after it
        if d.is_dir().await {
            d.push(s.file_name().unwrap());
        }
        // Run the checks
        if check_all(&s, &d).await.is_err() {
            return;
        }
        // Check if the source is relative
        if s.is_relative() {
            s = PathBuf::from(s.file_name().unwrap());
        }
        // A function to symlink a file
        let symlink_fn = async {
            let result = symlink(&s, &d).await;
            if check_err("There was an error symlinking", result, Some(&s)).is_ok()
                && flags.verbose
            {
                println!("{} -> {}", s.display(), d.display());
            }
        };

        // Basically same logic as line 80-87
        if d.exists().await {
            if flags.interactive {
                if prompt(&d) {
                    let _ = async_std::fs::remove_file(&d).await;
                    symlink_fn.await;
                }
            } else {
                let _ = async_std::fs::remove_file(&d).await;
                symlink_fn.await;
            }
        } else {
            symlink_fn.await;
        }
    }
}