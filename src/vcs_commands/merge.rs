use crate::util::archiving::unzip;
use crate::util::commit::Commit;
use crate::util::hash::VcsHash;
use crate::util::snapshot::{find_changes, restore_from_snapshot};
use crate::util::vcs_state::VcsState;
use indoc::indoc;
use std::collections::HashMap;
use std::fs::{copy, create_dir, remove_dir_all, remove_file};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// Merges the provided branch to master
pub fn run(repos_root: &Path, branch: &str) -> Result<String> {
    let mut state = VcsState::load(&repos_root)?;

    // branches[0] is always master branch
    let last_master_hash = state.branches[0].commit_hash;
    if last_master_hash != state.head {
        return Err(Error::new(
            ErrorKind::Other,
            indoc! {
            "The merge is possible only when you are in the last commit in master.
            Aborting..."}
            .to_owned(),
        ));
    }
    let hash_branch = match state.branches.iter().find(|x| branch == x.name) {
        Some(x) => x,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "error: wrong branch name",
            ))
        }
    }
    .commit_hash;
    let hash_ancestor = {
        let commit_by_hash = HashMap::<VcsHash, Commit>::from_iter(
            state
                .commits
                .iter()
                .map(|commit| (commit.hash, commit.clone())),
        );
        let mut commit = &commit_by_hash[&hash_branch];
        while commit.branch_name != "master" {
            commit = &commit_by_hash[&commit.parent];
        }
        commit.hash
    };

    {
        let changes = find_changes(&repos_root, &state.head)?;
        if !changes.is_empty() {
            let mut result =
                "Your local changes to the following files should be commited or dropped:\n"
                    .to_owned();
            for rel_path in (changes.modified.iter())
                .chain(changes.added.iter())
                .chain(changes.deleted.iter())
            {
                result.extend(format!("  {}\n", rel_path).chars());
            }
            result.extend(
                indoc! {
                "Please commit your changes or drop them before you jump.
                Aborting..."}
                .chars(),
            );
            return Err(Error::new(ErrorKind::Other, result));
        }
    }

    let changes_master = find_changes(&repos_root, &hash_ancestor)?;
    let changes_master_hashes = HashMap::<String, VcsHash>::from_iter(
        changes_master
            .added
            .iter()
            .map(|path_str| {
                (
                    path_str.clone(),
                    // unwrap: file should exist
                    VcsHash::from_file(&repos_root.join(path_str)).unwrap(),
                )
            })
            .chain(changes_master.modified.iter().map(|path_str| {
                (
                    path_str.clone(),
                    // unwrap: file should exist
                    VcsHash::from_file(&repos_root.join(path_str)).unwrap(),
                )
            }))
            .chain(
                changes_master
                    .deleted
                    .iter()
                    .map(|path_str| (path_str.clone(), VcsHash::zero())),
            ),
    );

    restore_from_snapshot(&repos_root, &hash_branch)?;
    // now the repository is branch snapshot
    let changes_branch = find_changes(&repos_root, &hash_ancestor)?;
    let changes_branch_hashes = HashMap::<String, VcsHash>::from_iter(
        changes_branch
            .added
            .iter()
            .map(|path_str| {
                (
                    path_str.clone(),
                    // unwrap: file should exist
                    VcsHash::from_file(&repos_root.join(path_str)).unwrap(),
                )
            })
            .chain(changes_branch.modified.iter().map(|path_str| {
                (
                    path_str.clone(),
                    // unwrap: file should exist
                    VcsHash::from_file(&repos_root.join(path_str)).unwrap(),
                )
            }))
            .chain(
                changes_branch
                    .deleted
                    .iter()
                    .map(|path_str| (path_str.clone(), VcsHash::zero())),
            ),
    );

    let mut changes_intersection = Vec::<String>::new();
    for (path, hash1) in changes_master_hashes {
        if let Some(&hash2) = changes_branch_hashes.get(&path) {
            if hash1 != hash2 {
                changes_intersection.push(path.clone());
            }
        }
    }

    if !changes_intersection.is_empty() {
        let mut result =
            "Merge confilict: file has been changed both in master and branch\n".to_owned();
        for rel_path in changes_intersection {
            result.extend(format!("  {}\n", rel_path).chars());
        }
        result.extend("Aborting...".chars());
        restore_from_snapshot(&repos_root, &state.head)?;
        return Ok(result);
    }
    let snapshot_master = repos_root.join(".vcs").join("snapshot_master");
    let archive_master = repos_root
        .join(".vcs")
        .join("snapshots")
        .join(format!("{}.zip", state.head.to_string()));
    create_dir(&snapshot_master)?;
    unzip(&archive_master, &snapshot_master)?;

    for rel_path in changes_master
        .modified
        .iter()
        .chain(changes_master.added.iter())
    {
        let to = repos_root.join(&rel_path);
        if !to.try_exists()? {
            copy(snapshot_master.join(rel_path), to)?;
        }
    }
    remove_dir_all(snapshot_master)?;

    for rel_path in changes_master.deleted {
        let path = repos_root.join(rel_path);
        if path.try_exists()? {
            remove_file(path)?;
        }
    }
    // unwrap: assume the state.branches is valid
    let pos = state
        .branches
        .iter()
        .position(|x| branch == x.name)
        .unwrap();
    state.branches.remove(pos);
    for commit in state.commits.iter() {
        if commit.branch_name == branch {
            let snapshot_path = repos_root
                .join(".vcs")
                .join("snapshots")
                .join(format!("{}.zip", commit.hash.to_string()));
            remove_file(&snapshot_path)?;
        }
    }
    state.commits = state
        .commits
        .into_iter()
        .filter(|x| x.branch_name != branch)
        .collect();

    state.update_vcs_dir(&repos_root)?;
    let mut result = "Successfully created merge commit:\n".to_owned();
    result.extend(super::commit::run(repos_root, &format!("Merged branch {}.", branch))?.chars());
    result.extend(format!("\nDeleted {}", branch).chars());
    Ok(result)
}
