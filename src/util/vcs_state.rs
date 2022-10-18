use super::hash::VcsHash;
use super::{branch::Branch, commit::Commit};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Contains the full state of the repository
#[derive(Deserialize, Serialize)]
pub struct VcsState {
    pub head: VcsHash,
    pub branch_name: String,
    pub branches: Vec<Branch>,
    pub commits: Vec<Commit>,
}

/// Finds the closest ancestor directory containing ".vcs" folder
pub fn find_repos_root() -> Result<PathBuf, Error> {
    for path in current_dir()?.ancestors() {
        let path_dir = path.join(".vcs");
        if path_dir.try_exists()? && path_dir.is_dir() {
            return Ok(path.to_owned());
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "error: \".vcs\" is not achievable",
    ))
}

impl VcsState {
    /// Loads VCS state from "<repos_root>/.vcs"
    pub fn load(repos_root: &Path) -> Result<Self, Error> {
        let file = File::open(repos_root.join(".vcs").join("status.json"))?;
        Ok(serde_json::from_reader(BufReader::new(file))?)
    }

    /// Updates the VCS state at "<repos_root>/.vcs/"
    pub fn update_vcs_dir(&self, repos_root: &Path) -> Result<(), Error> {
        let file = std::fs::File::create(repos_root.join(".vcs").join("status.json"))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, self)?)
    }

    /// Initializes VCS at "<repos_root>/.vcs" without any commits
    pub fn init(repos_root: &Path) -> Result<VcsState, Error> {
        let path = repos_root.join(".vcs");
        create_dir(&path)?;
        create_dir(path.join("snapshots"))?;
        let file = std::fs::File::create(repos_root.join(".vcs").join("status.json"))?;
        let writer = std::io::BufWriter::new(file);

        let state = VcsState {
            head: VcsHash::zero(),
            branch_name: "master".to_owned(),
            branches: vec![],
            commits: vec![],
        };
        serde_json::to_writer_pretty(writer, &state)?;
        Ok(state)
    }

    pub fn assert_validity(repos_path: &Path) {
        use std::collections::{HashMap, HashSet};

        let state = VcsState::load(&repos_path).unwrap();
        let commits: HashMap<VcsHash, &Commit> =
            HashMap::from_iter(state.commits.iter().map(|x| (x.hash, x)));
        let branches: HashSet<&String> = HashSet::from_iter(state.branches.iter().map(|x| &x.name));
        assert!(commits.get(&state.head).is_some());
        assert_eq!(state.commits[0].parent, VcsHash::zero());
        assert!(!state.branches.is_empty());
        assert_eq!(state.branches[0].name, "master");
        for commit in state.commits {
            assert!(branches.contains(&commit.branch_name));
            let snapshot_path = repos_path
                .join(".vcs")
                .join("snapshots")
                .join(format!("{}.zip", commit.hash.to_string()));
            assert!(snapshot_path.try_exists().unwrap());
        }
    }
}
