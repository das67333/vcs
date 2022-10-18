use crate::util::snapshot::find_changes;
use crate::util::vcs_state::VcsState;
use std::io::Error;
use std::path::Path;

/// Shows the working tree status
pub fn run(repos_root: &Path) -> Result<String, Error> {
    let state = VcsState::load(&repos_root)?;
    let changes = find_changes(&repos_root, &state.head)?;
    if changes.is_empty() {
        return Ok("No changes to be committed".to_owned());
    }
    let mut result = format!("On branch {}\n", state.branch_name);
    result.extend("Changes to be committed:\n".chars());

    for file in changes.modified {
        result.extend(format!("  modified: {file}\n").chars());
    }
    for file in changes.added {
        result.extend(format!("  new file: {file}\n").chars());
    }
    for file in changes.deleted {
        result.extend(format!("  deleted:  {file}\n").chars());
    }
    result.pop();
    Ok(result)
}
