const CURR_BR_IS_MASTER: bool = true;

pub fn run(name: &str) -> String {
    if CURR_BR_IS_MASTER {
        format!(
            "Created a new branch {} from master's commit <commit_hash>",
            name
        )
    } else {
        format!(
            "Creating a new branch is possible only when you are in the master branch.
Aborting..."
        )
    }
}
