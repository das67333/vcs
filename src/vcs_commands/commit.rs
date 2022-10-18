use crate::util::commit::Commit;
use crate::util::snapshot::{create_snapshot, find_changes};
use crate::util::vcs_state::VcsState;
use chrono::Local;
use indoc::indoc;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Commits changes in the working tree if any
pub fn run(repos_root: &Path, message: &str) -> Result<String, Error> {
    let mut state = VcsState::load(&repos_root)?;
    // unwrap: assume the state.branch_name is valid
    let branch = state
        .branches
        .iter_mut()
        .find(|branch| branch.name == state.branch_name)
        .unwrap();
    if branch.commit_hash != state.head {
        return Err(Error::new(
            ErrorKind::Other,
            indoc! {
            "You can create a new commit only from last one of the branch.
            Aborting.."}
            .to_owned(),
        ));
    }
    let changes = find_changes(&repos_root, &state.head)?;
    if changes.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "No changes to be committed".to_owned(),
        ));
    }
    let hash = create_snapshot(&repos_root)?;

    let mut result = format!("[{} {}] {}\n", state.branch_name, hash.short_str(), message);
    // constructing a row like: "3 files changed, 1 added"
    {
        let mut to_join = vec![];
        if !changes.modified.is_empty() {
            to_join.push((changes.modified.len(), "changed"));
        }
        if !changes.added.is_empty() {
            to_join.push((changes.added.len(), "added"));
        }
        if !changes.deleted.is_empty() {
            to_join.push((changes.deleted.len(), "deleted"));
        }
        let row: String = (0..to_join.len())
            .into_iter()
            .map(|i| {
                if i == 0 {
                    format!(
                        "  {} {} {}",
                        to_join[i].0,
                        if to_join[i].0 == 1 { "file" } else { "files" },
                        to_join[i].1
                    )
                } else {
                    format!(", {} {}", to_join[i].0, to_join[i].1)
                }
            })
            .collect();
        result.extend(format!("{}\n", row).chars());
    }
    for file in changes.modified.iter() {
        result.extend(format!("  modified {file}\n").chars());
    }
    for file in changes.added.iter() {
        result.extend(format!("  added {file}\n").chars());
    }
    for file in changes.deleted.iter() {
        result.extend(format!("  deleted {file}\n").chars());
    }
    result.pop();

    branch.commit_hash = hash;
    let commit = Commit {
        branch_name: branch.name.clone(),
        time: Local::now(),
        message: message.to_owned(),
        changes,
        hash,
        parent: state.head,
    };
    state.head = hash;
    state.commits.push(commit);
    state.update_vcs_dir(&repos_root)?;
    Ok(result)
}
