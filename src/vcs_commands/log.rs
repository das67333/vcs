use crate::util::hash::VcsHash;
use crate::util::vcs_state::VcsState;
use indoc::indoc;
use std::io::Error;
use std::path::Path;

/// Lists the current commit's ancestors
pub fn run(repos_root: &Path) -> Result<String, Error> {
    let state = VcsState::load(&repos_root)?;
    let mut hash = state.head;
    let mut result = String::new();
    while hash != VcsHash::zero() {
        // unwrap: assume the state.commits is valid
        let commit = state.commits.iter().find(|&x| hash == x.hash).unwrap();
        let changes = if commit.changes.is_empty() {
            "No changes\n".to_owned()
        } else {
            let mut temp = "Changes\n".to_owned();
            for file in commit.changes.modified.iter() {
                temp.extend(format!("  modified {file}\n").chars());
            }
            for file in commit.changes.added.iter() {
                temp.extend(format!("  added {file}\n").chars());
            }
            for file in commit.changes.deleted.iter() {
                temp.extend(format!("  deleted {file}\n").chars());
            }
            temp
        };

        result.extend(
            format!(
                indoc! {"commit {}
                Date: {}
                Message: {}
                {}\n"},
                commit.hash,
                commit.time.format("%a %b %-e %X %Y %z"),
                commit.message,
                changes
            )
            .chars(),
        );
        hash = commit.parent;
    }
    result.pop();
    result.pop();
    Ok(result)
}
