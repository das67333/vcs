use crate::util::branch::Branch;
use crate::util::commit::Commit;
use crate::util::hash::CommitHash;
use crate::util::snapshot;
use crate::util::vcs_state::VcsState;
use std::io::Result;
use std::path::{Path, MAIN_SEPARATOR};

pub fn run(path: &str) -> Result<String> {
    let path = Path::new(path).canonicalize()?;
    let mut state = VcsState::init(&path)?;
    let commit_hash = snapshot::create(&path)?;
    let commit = Commit {
        branch_name: "master".to_owned(),
        message: "Initial commit".to_owned(),
        // changes: "",
        hash: commit_hash,
        parent: CommitHash::zero(),
    };
    let branch = Branch {
        name: "master".to_owned(),
        commit_hash,
    };
    state.head = Some(commit_hash);
    state.commits.push(commit);
    state.branches.push(branch);
    state.update_vcs_dir(&path)?;

    Ok(format!(
        "Initialized VCS repository in {}{}
    Created commit:
    [master {}] Initial commit",
        // unwrap: "path" must be at least Unicode
        path.to_str().unwrap(),
        MAIN_SEPARATOR,
        commit_hash.short_str()
    ))
}
