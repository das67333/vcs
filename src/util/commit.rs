use super::hash::VcsHash;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Stores commit data
#[derive(Deserialize, Serialize, Clone)]
pub struct Commit {
    pub branch_name: String,
    pub time: DateTime<Local>,
    pub message: String,
    pub changes: CommitChanges,
    pub hash: VcsHash,
    pub parent: VcsHash,
}

/// Represents changes in the working tree
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct CommitChanges {
    pub modified: Vec<String>,
    pub added: Vec<String>,
    pub deleted: Vec<String>,
}

impl CommitChanges {
    pub fn is_empty(&self) -> bool {
        self.modified.is_empty() && self.added.is_empty() && self.deleted.is_empty()
    }
}
