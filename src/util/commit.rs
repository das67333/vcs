use super::hash::CommitHash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Commit {
    pub branch_name: String,
    pub message: String,
    // pub changes: String,
    pub hash: CommitHash,
    pub parent: CommitHash,
}
