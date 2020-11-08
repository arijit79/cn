use crate::{utils::senderr, Abort};
use async_std::fs::File;
use async_std::path::PathBuf;

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
