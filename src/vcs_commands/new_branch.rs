use crate::util::branch::Branch;
use crate::util::vcs_state::VcsState;
use indoc::indoc;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Creates a new branch with the given name
pub fn run(repos_root: &Path, name: &str) -> Result<String, Error> {
    let mut state = VcsState::load(&repos_root)?;
    if state.branch_name != "master" {
        return Err(Error::new(
            ErrorKind::Other,
            indoc! {
            "Creating a new branch is possible only when you are in the master branch.
            Aborting..."}
            .to_owned(),
        ));
    }
    if state.branches.iter().find(|&x| name == x.name).is_some() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                indoc! {
                "Branch {} already exists.
                Aborting..."},
                name
            ),
        ));
    }
    state.branches.push(Branch {
        name: name.to_owned(),
        commit_hash: state.head,
    });
    state.branch_name = name.to_owned();
    state.update_vcs_dir(&repos_root)?;
    Ok(format!(
        "Created a new branch {} from master's commit {}",
        name, state.head.short_str()
    ))
}
