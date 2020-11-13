use crate::{utils::senderr, Abort};
use async_std::fs::File;
use async_std::path::PathBuf;
use std::process::exit;

pub async fn check_all(s: &PathBuf, d: &PathBuf) -> Result<(), Abort> {
    // Check if source exists and it can be read
    if !s.exists().await {
        senderr(format!("'{}' No such file or directory", s.display()));
        return Err(Abort);
    }

    // Check if we have read the file
    if File::open(&s).await.is_err() {
        senderr(format!("'{}' Permission denied", s.display()));
        return Err(Abort);
    }

    // Get the destination file's directory
    // NOTE: Use `.is_empty()` here because parent may return an empty string in `Some`, even if there is no parent
    let check_dir = if d.parent().unwrap().as_os_str().is_empty() {
        // If there is nothing, then consider the current directory
        PathBuf::from("./")
    } else {
        d.parent().unwrap().to_path_buf()
    };

    // Check destination write permissions
    if check_dir.metadata().await.unwrap().permissions().readonly() {
        senderr(format!("'{}' Permission denied", s.display()));
        exit(1);
    }

    // Check if source is not same as the destination
    if s == d {
        senderr(format!(
            "source '{}': is same as the destination '{}'",
            s.display(),
            d.display()
        ));
        return Err(Abort);
    }
    Ok(())
}

// Check if two paths' canonicalize to the same path
pub fn check_canonical(s: &PathBuf, d: &PathBuf) {
    let mut source_path = std::env::current_dir().unwrap();
    source_path.push(s.parent().unwrap());
    let mut dest_path = std::env::current_dir().unwrap();
    dest_path.push(d.parent().unwrap());
    if source_path.canonicalize().unwrap() != dest_path.canonicalize().unwrap() {
        // If they don't match, throw an error
        senderr("Source and destination must be in the same directory when creating relative symbolic links".to_string());
        exit(2);
    }
}
