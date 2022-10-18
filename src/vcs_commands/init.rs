use crate::util::branch::Branch;
use crate::util::commit::{Commit, CommitChanges};
use crate::util::hash::VcsHash;
use crate::util::snapshot::create_snapshot;
use crate::util::vcs_state::VcsState;
use chrono::Local;
use indoc::indoc;
use std::fs::create_dir;
use std::io::Error;
use std::path::{Path, MAIN_SEPARATOR};

/// Initializes VCS repository at the given path
pub fn run(path: &Path) -> Result<String, Error> {
    if !path.try_exists()? {
        create_dir(&path)?;
    }
    let mut state = VcsState::init(&path)?;
    let commit_hash = create_snapshot(&path)?;
    let commit = Commit {
        branch_name: state.branch_name.clone(),
        time: Local::now(),
        message: "Initial commit".to_owned(),
        changes: CommitChanges::default(),
        hash: commit_hash,
        parent: VcsHash::zero(),
    };
    let branch = Branch {
        name: state.branch_name.clone(),
        commit_hash,
    };
    state.head = commit_hash;
    state.commits.push(commit);
    state.branches.push(branch);
    state.update_vcs_dir(&path)?;

    Ok(format!(
        indoc! {
        "Initialized VCS repository in {}{}
        Created commit:
        [{} {}] Initial commit"},
        // unwrap: assume the "path" is at least Unicode
        path.to_str().unwrap(),
        MAIN_SEPARATOR,
        state.branch_name,
        commit_hash.short_str()
    ))
}
