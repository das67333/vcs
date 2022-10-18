use super::archiving::{unzip, zip};
use super::commit::CommitChanges;
use super::hash::VcsHash;
use std::collections::HashSet;
use std::fs::{create_dir, remove_dir_all, remove_file};
use std::io::Error;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Creates a snapshot of the current state of repository
pub fn create_snapshot(repos_root: &Path) -> Result<VcsHash, Error> {
    let archive_path = repos_root.join(".vcs").join("snapshots").join("temp.zip");
    zip(&repos_root, &archive_path)?;
    let hash = VcsHash::from_file(&archive_path)?;
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
pub fn restore_from_snapshot(repos_root: &Path, commit_hash: &VcsHash) -> Result<(), Error> {
    let archive_path = repos_root
        .join(".vcs")
        .join("snapshots")
        .join(format!("{}.zip", commit_hash.to_string()));

    for path in WalkDir::new(&repos_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().to_owned())
        .filter(|x| x.is_file())
    {
        // unwrap: "repos_root" is always a prefix of "path"
        if path
            .components()
            .any(|x| x.as_os_str().to_str() == Some(".vcs"))
        {
            continue;
        }
        remove_file(path)?;
    }
    unzip(&archive_path, repos_root)?;
    Ok(())
}

/// Compares the repository with the snapshot from commit_hash.
pub fn find_changes(repos_root: &Path, commit_hash: &VcsHash) -> Result<CommitChanges, Error> {
    let snapshot_root = repos_root.join(".vcs").join("snapshot_to_compare");
    create_dir(&snapshot_root)?;
    let archive_path = repos_root
        .join(".vcs")
        .join("snapshots")
        .join(format!("{}.zip", commit_hash.to_string()));
    unzip(&archive_path, &snapshot_root)?;
    let mut snapshot_paths = HashSet::<PathBuf>::new();
    for entry in WalkDir::new(&snapshot_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        // unwrap: "temp_path" is always a prefix of "path"
        let rel_path = path.strip_prefix(&snapshot_root).unwrap().to_owned();
        if path.is_file() {
            snapshot_paths.insert(rel_path);
        }
    }
    let mut repos_paths = HashSet::<PathBuf>::new();
    for entry in WalkDir::new(&repos_root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // unwrap: "temp_path" is always a prefix of "path"
        let rel_path = path.strip_prefix(&repos_root).unwrap();
        if rel_path
            .components()
            .any(|x| x.as_os_str().to_str() == Some(".vcs"))
        {
            continue;
        }
        if path.is_file() {
            repos_paths.insert(rel_path.to_owned());
        }
    }
    let mut changes = CommitChanges::default();
    for rel_path in snapshot_paths {
        if repos_paths.contains(&rel_path) {
            let hash_old = VcsHash::from_file(&snapshot_root.join(&rel_path))?;
            let hash_new = VcsHash::from_file(&repos_root.join(&rel_path))?;
            if hash_old != hash_new {
                // unwrap: os_str is always at least Unicode
                changes.modified.push(rel_path.to_str().unwrap().to_owned());
            }
        } else {
            // unwrap: os_str is always at least Unicode
            changes.deleted.push(rel_path.to_str().unwrap().to_owned());
        }
        repos_paths.remove(&rel_path);
    }
    for rel_path in repos_paths {
        // unwrap: os_str is always at least Unicode
        changes.added.push(rel_path.to_str().unwrap().to_owned());
    }
    changes.modified.sort();
    changes.added.sort();
    changes.deleted.sort();
    remove_dir_all(&snapshot_root)?;
    Ok(changes)
}
