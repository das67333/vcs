pub fn run(_message: &str) -> String {
    if is_commit_last() {
        format!(
"[<branch_name> <commit_hash>] Work in progress                                               
\t3 files changed, 1 added
\tmodified path/to/modified/file.rs
\tmodified path/to/modified/file2.rs
\tadded path/to/new/file.rs")
    } else {
        format!(
            "You can create a new commit only from last one.
Aborting..."
        )
    }
}

fn is_commit_last() -> bool {
    true
}
