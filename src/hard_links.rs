use async_std::fs::{hard_link, remove_file, remove_dir_all};
use async_std::path::PathBuf;
use crate::{checks::check_all, utils::{senderr, prompt}, args::Flags};

pub async fn hl_item(sources: Vec<PathBuf>, dest: PathBuf, flags: &Flags) {
    if sources.len() > 1 {
        senderr(format!("Only one source could be given when making hard links"))
    }
    let dest_clone = dest.clone();
    let async_remove = async {
        if dest_clone.is_dir().await {
            let _ = remove_dir_all(&dest_clone).await;
        } else {
            let _ = remove_file(&dest_clone).await;
        }};
    if dest_clone.exists().await && flags.interactive {
        if prompt(&dest_clone) {
            async_remove.await;
        }
    } else {
        async_remove.await;
    }
    let source = &sources[0];
    let dest_check = if dest.parent().unwrap().to_str().unwrap().is_empty() {
        PathBuf::from("./")
    } else {
        dest.parent().unwrap().to_owned()
    };
    let basic_checks = check_all(&source, &dest_check).await;
    
    if basic_checks.is_err() {
        std::process::exit(1);
    } else if !source.is_file().await {
        senderr("Source must be a regular file".to_string())
    }
    let _ = hard_link(source, dest).await;
}