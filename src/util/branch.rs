use super::hash::VcsHash;
use serde::{Deserialize, Serialize};

/// Stores state of a branch
#[derive(Deserialize, Serialize)]
pub struct Branch {
    pub name: String,
    pub commit_hash: VcsHash,
}
