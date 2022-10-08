use super::hash::CommitHash;
use super::{branch::Branch, commit::Commit};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::io::{BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct VcsState {
    pub head: Option<CommitHash>,
    pub branches: Vec<Branch>,
    pub commits: Vec<Commit>,
}

/// Find closest ancestor directory containing ".vcs" folder
pub fn find_repos_root() -> Result<PathBuf> {
    // unwrap: "." is always a valid directory
    for path in Path::new(".").canonicalize().unwrap().ancestors() {
        let path = path.join(".vcs");
        if path.exists() && path.is_dir() {
            return Ok(path.to_owned());
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "\".vcs\" is not achievable",
    ))
}

impl VcsState {
    /// Load VCS state from "<repos_root>/.vcs"
    pub fn load(repos_root: &Path) -> Result<Self> {
        let file = File::open(repos_root.join(".vcs").join("status.json"))?;
        Ok(serde_json::from_reader(BufReader::new(file))?)
    }

    /// Update VCS state at "<repos_root>/.vcs/"
    pub fn update_vcs_dir(&self, repos_root: &Path) -> Result<()> {
        let file = std::fs::File::create(repos_root.join(".vcs").join("status.json"))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, self)?)
    }

    /// Init VCS at "<repos_root>/.vcs" without initial commit
    pub fn init(repos_root: &Path) -> Result<VcsState> {
        let path = repos_root.join(".vcs");
        create_dir(&path)?;
        create_dir(path.join("snapshots"))?;
        let file = std::fs::File::create(repos_root.join(".vcs").join("status.json"))?;
        let writer = std::io::BufWriter::new(file);

        let state = VcsState {
            head: None,
            branches: vec![],
            commits: vec![],
        };
        serde_json::to_writer_pretty(writer, &state)?;
        Ok(state)
    }
}
