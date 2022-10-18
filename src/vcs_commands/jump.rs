use crate::util::commit::Commit;
use crate::util::snapshot::{find_changes, restore_from_snapshot};
use crate::util::vcs_state::VcsState;
use indoc::indoc;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// Changes the state of the working tree according to the given arguments
pub fn run(
    repos_root: &Path,
    branch_name: &Option<String>,
    commit_hash: &Option<String>,
) -> Result<String> {
    let mut state = VcsState::load(&repos_root)?;
    let changes = find_changes(&repos_root, &state.head)?;
    if !changes.is_empty() {
        let mut result =
            "error: Your local changes to the following files should be commited or dropped:\n"
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
    if branch_name.is_some() && commit_hash.is_some() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "impossible error: both of the jump arguments provided",
        ));
    }
    if branch_name.is_none() && commit_hash.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "impossible error: none of the jump arguments provided",
        ));
    }
    let result;
    let hash = if let Some(name) = branch_name {
        let branch = state
            .branches
            .iter_mut()
            .find(|branch| &branch.name == name);
        if let Some(branch) = branch {
            result = format!(
                "Successfully jumped to branch {}. Current commit: {}",
                name,
                branch.commit_hash.short_str()
            );
            if name == &state.branch_name && branch.commit_hash == state.head {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Already on branch {}", name),
                ));
            }
            state.branch_name = name.clone();
            branch.commit_hash
        } else {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    indoc! {"No branch {} exists.
                    Aborting..."},
                    name
                ),
            ));
        }
    } else if let Some(hash) = commit_hash {
        if hash.len() > 40 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "error: provided hash prefix is longer than 40 symbols",
            ));
        }
        let matching_commits: Vec<&Commit> = state
            .commits
            .iter()
            .filter(|x| hash[..] == x.hash.to_string()[..hash.len()])
            .collect();
        if matching_commits.len() == 0 {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    indoc! {
                    "No commit with hash {} exists.
                    Aborting..."},
                    hash
                ),
            ));
        }
        if matching_commits.len() > 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "hash is ambiguous: hash prefix matches several hashes",
            ));
        }
        if matching_commits[0].hash == state.head {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Already on commit {}", state.head.short_str()),
            ));
        }
        state.branch_name = matching_commits[0].branch_name.clone();
        result = format!(
            "Successfully jumped to commit {}. Current branch: {}",
            matching_commits[0].hash.short_str(),
            state.branch_name
        );
        matching_commits[0].hash
    } else {
        // impossible panic
        panic!()
    };
    state.head = hash;
    restore_from_snapshot(&repos_root, &hash)?;
    state.update_vcs_dir(&repos_root)?;
    Ok(result)
}
