use super::hash::CommitHash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Branch {
    pub name: String,
    pub commit_hash: CommitHash,
}
