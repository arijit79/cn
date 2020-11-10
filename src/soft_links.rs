use crate::{checks::{check_all, check_canonical}, utils::{senderr, prompt, check_err}, args::Flags};
use async_std::{path::PathBuf, fs::create_dir};
use futures::future::join_all;
use std::process::exit;
use async_std::prelude::*;

#[cfg(target_family = "unix")]
pub mod unix_symlink {
    use super::*;
    use async_std::os::unix::fs::symlink;

    pub async fn sl_item(sources: Vec<PathBuf>, dest: PathBuf, flags: &Flags) {
        if dest.is_file().await {
            sl_file(sources[0].clone(), dest.clone(), flags).await;
        } else {
            let mut tasks = vec![];
            for i in sources {
                if i.is_dir().await {
                    if i.is_relative() || dest.is_relative() {
                        senderr("Relative paths are not allowed when symlinking directories");
                        exit(2);
                    }
                    if i.is_relative() ^ dest.is_relative() {
                        senderr("Absolute and relative paths cannot be used simultaniously");
                        exit(2);
                    }
                    tasks.push(sl_dir(i, dest.clone(), flags));
                } else {
                    if i.is_relative() && dest.is_relative() {
                        check_canonical(&i, &dest);
                    } else if i.is_relative() ^ dest.is_relative() {
                        senderr("Absolute and relative paths cannot be used simultaniously");
                        exit(2);
                    }
                    tasks.push(
                        Box::pin(sl_file(i, dest.clone(), flags))
                    );
                }
            }
            join_all(tasks).await;
        }
    }

    #[async_recursion::async_recursion]
    pub async fn sl_dir(s: PathBuf, mut d: PathBuf, flags: &Flags) {
        if d.parent().unwrap().metadata().await.unwrap().permissions().readonly() {
            senderr(format!("`{}` Permission denied", d.display()));
            return;
        }
        d.push(s.file_name().unwrap());

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

        if d.exists().await {
            if flags.interactive {
                if prompt(&d) {
                    let _ = async_std::fs::remove_dir_all(&d).await;
                    init_dir.await;
                }
            } else {
                let _ = async_std::fs::remove_dir_all(&d).await;
                init_dir.await;
            }
        } else {
            init_dir.await;
        }
        
        let mut tasks = vec![];
        let mut entries = async_std::fs::read_dir(&s).await.unwrap();
        while let Some(p) = entries.next().await {
            let name = p.unwrap().file_name();
            let mut source = s.clone();
            source.push(&name);
            if source.is_dir().await {
                tasks.push(sl_dir(source, d.clone(), flags));
            } else {
                tasks.push(Box::pin(sl_file(source, d.clone(), flags)));
            }
        }
        join_all(tasks).await;
    }

    pub async fn sl_file(mut s: PathBuf, mut d: PathBuf, flags: &Flags) {
        if d.is_dir().await {
            d.push(s.file_name().unwrap());
        }

        let check_dir = if d.parent().unwrap().to_str().unwrap().is_empty() {
            PathBuf::from("./")
        } else {
            d.parent().unwrap().to_path_buf()
        };
        if check_all(&s, &check_dir).await.is_err() {
            return;
        }
        if s.is_relative() {
            s = PathBuf::from(s.file_name().unwrap());
        }

        let symlink_fn = async {
            let result = symlink(&s, &d).await;
            if check_err("There was an error symlinking", result, Some(&s)).is_err() && flags.verbose {
                println!("{} -> {}", s.display(), d.display());
            }
        };

        if d.exists().await {
            if flags.interactive  {
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