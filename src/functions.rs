use crate::args::Flags;
use crate::checks::check_all;
use crate::utils::prompt;
use async_std::fs::{copy, create_dir, read_dir, remove_file, remove_dir_all};
use async_std::prelude::*;
use async_std::path::PathBuf;
use futures::future::join_all;

#[async_recursion::async_recursion]
pub async fn copy_dir(s: PathBuf, dest: PathBuf, flags: &Flags) {
    let mut d = dest.clone();
    if d.is_dir().await { d.push(&s.file_name().unwrap()); }
    if flags.interactive && d.exists().await {
        if prompt(&d) {
            let _ = remove_dir_all(&d);
            if create_dir(&d).await.is_ok() && flags.verbose {
                println!("{} -> {}", s.display(), d.display())
            }
        }
    } else {
        if create_dir(&d).await.is_ok() && flags.verbose {
            println!("{} -> {}", s.display(), d.display())
        }
    }
    let mut dir_read = read_dir(&s).await.unwrap();
    let mut tasks = vec![];
    while let Some(item) = dir_read.next().await {
        let mut name = PathBuf::from(&s);
        name.push(item.unwrap().file_name());
        if name.is_dir().await {
            tasks.push(copy_dir(name, d.clone(), flags));
        } else {
            tasks.push(
                Box::pin(copy_file(name, d.clone(), flags))
            )
        }
    }
    join_all(tasks).await;
    let mut entries = read_dir(&s).await.unwrap();
    if !flags.copy && entries.next().await.is_none() {
        let _ = remove_dir_all(s).await;
    }
}

pub async fn copy_file(s: PathBuf, d: PathBuf, flags: &Flags) {
    // Get a copy of cdest and push the filename
    let mut d = d.clone();
    if d.is_dir().await {
        d.push(&s.file_name().unwrap());
    }
    // Check if the file passes all checks
    let checks = check_all(&s, &d).await;
    if checks.is_err() {
        return;
    }
    if flags.interactive && d.exists().await {
        if prompt(&d) {
            let result = copy(&s, &d).await;
            if !flags.copy && !result.is_err() {
                let _ = async_std::fs::remove_file(&s).await;
            } else if result.is_err() {
                result.unwrap_or_else(|r| {
                    eprintln!("Failed to copy...\n\n{}", r.to_string());
                    0
                });
            } else {
                if !flags.copy {
                    let _ = remove_file(&s).await;
                }
                if flags.verbose {
                    println!("{} -> {}", s.display(), d.display())
                }
            }
        }
    } else {
        let result = copy(&s, &d).await;
        if result.is_err() {
            result.unwrap_or_else(|r| {
                eprintln!("{:?}", r.kind());
                0
            });
        } else if !flags.copy {
            let _ = remove_file(&s).await;
        }
        if flags.verbose {
            println!("{} -> {}", s.display(), d.display())
        }
    }
}
