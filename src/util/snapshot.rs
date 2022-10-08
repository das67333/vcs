use super::archiving::{unzip, zip};
use super::hash::CommitHash;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::Result;
use std::path::Path;

/// Create a snapshot of current state of repository
pub fn create(repos_root: &Path) -> Result<CommitHash> {
    let archive_path = repos_root.join(".vcs").join("snapshots").join("temp.zip");
    zip(&repos_root, &archive_path)?;
    let hash = CommitHash::from_file(&archive_path)?;
    std::fs::rename(
        &archive_path,
        // unwrap: "archive_path" is definitely not "/"
        archive_path
            .parent()
            .unwrap()
            .join(format!("{}.zip", hash).as_str()),
    )?;
    Ok(hash)
}

/// Backup repository state from a snapshot
pub fn backup(repos_root: &Path, commit_hash: &CommitHash) -> Result<()> {
    let archive_path = repos_root
        .join(".vcs")
        .join("snapshots")
        .join(format!("{}.zip", commit_hash.to_string()));

    for entry in read_dir(repos_root)? {
        let path = entry?.path();
        if path.strip_prefix(&repos_root).unwrap().as_os_str().to_str() == Some(".vcs") {
            continue;
        }
        if path.is_dir() {
            remove_dir_all(path)?;
        } else {
            remove_file(path)?;
        }
    }
    unzip(&archive_path, repos_root)?;
    Ok(())
}

// Compare the repository with the latest snapshot. if it does not exist, it is considered empty
// pub fn find_changes() -> Result<String> {
//     let mut result = String::new();

//     Ok("".to_owned())
// }
