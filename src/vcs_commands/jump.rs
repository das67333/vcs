const UNCHANGED: bool = true;

pub fn run(branch_name: &Option<String>, commit_hash: &Option<String>) -> String {
    if branch_name.is_some() && commit_hash.is_some() {
        panic!("impossible error - both of the jump arguments provided")
    }
    if let Some(name) = branch_name {
        run_by_branch(&name)
    } else if let Some(hash) = commit_hash {
        run_by_commit(&hash)
    } else {
        panic!("impossible error - none of the jump arguments provided")
    }
}

pub fn run_by_branch(branch_name: &str) -> String {
    if UNCHANGED {
        format!(
            "Successfully jumped to branch {}. Current commit: <commit_hash>.",
            branch_name
        )
    } else {
        format!(
            "error: Your local changes to the following files should be commited or dropped:
\tpath/to/modified/file2.rs
\tpath/to/new/file.rs
Please commit your changes or drop them before you jump.
Aborting..."
        )
    }
}

pub fn run_by_commit(commit_hash: &str) -> String {
    if UNCHANGED {
        format!(
            "Successfully jumped to commit {}. Current branch: <branch_name>.",
            commit_hash
        )
    } else {
        format!(
            "error: Your local changes to the following files should be commited or dropped:
\tpath/to/modified/file2.rs
\tpath/to/new/file.rs
Please commit your changes or drop them before you jump.
Aborting..."
        )
    }
}
