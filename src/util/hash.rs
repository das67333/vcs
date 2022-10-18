use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use sha1::{Digest, Sha1};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;

/// SHA-1 hash for identifying and comparing commmits and file contents
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VcsHash([u8; 20]);

impl Serialize for VcsHash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl Debug for VcsHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<'de> Deserialize<'de> for VcsHash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.len() != 40 {
            return Err(D::Error::custom(format!(
                "hash is corrupted: {} has length {} while 40 is expected",
                &s,
                s.len()
            )));
        }
        // unwrap: 16 is in range 2..=36
        let arr: Vec<u8> = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect();
        // unwrap: s.len() is 40 => arr.len() is 20
        Ok(Self(arr.try_into().unwrap()))
    }
}

impl Display for VcsHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.0 {
            write!(f, "{:02x}", x)?;
        }
        write!(f, "")
    }
}

impl VcsHash {
    pub fn zero() -> Self {
        VcsHash([0; 20])
    }

    /// Calculates SHA-1 hash from file contents
    pub fn from_file(path: &Path) -> Result<VcsHash, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut hasher = Sha1::new();
        hasher.update(buffer);
        // unwrap: length is always 20
        let slice = hasher.finalize().to_vec().try_into().unwrap();
        Ok(VcsHash(slice))
    }

    /// Shortens hash string representation
    pub fn short_str(&self) -> String {
        self.to_string()[..6].to_owned()
    }
}
