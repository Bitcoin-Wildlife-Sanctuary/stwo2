use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::Digest;

// Wrapper for the sha256 hash type.
#[repr(align(32))]
#[derive(Clone, Copy, PartialEq, Default, Eq, Deserialize, Serialize)]
pub struct BWSSha256Hash(pub(crate) [u8; 32]);

impl From<BWSSha256Hash> for Vec<u8> {
    fn from(value: BWSSha256Hash) -> Self {
        Vec::from(value.0)
    }
}

impl From<Vec<u8>> for BWSSha256Hash {
    fn from(value: Vec<u8>) -> Self {
        Self(
            value
                .try_into()
                .expect("Failed converting Vec<u8> to BWSSha256Hash type"),
        )
    }
}

impl From<&[u8]> for BWSSha256Hash {
    fn from(value: &[u8]) -> Self {
        Self(
            value
                .try_into()
                .expect("Failed converting &[u8] to BWSSha256Hash Type!"),
        )
    }
}

impl AsRef<[u8]> for BWSSha256Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<BWSSha256Hash> for [u8; 32] {
    fn from(val: BWSSha256Hash) -> Self {
        val.0
    }
}

impl fmt::Display for BWSSha256Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl fmt::Debug for BWSSha256Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <BWSSha256Hash as fmt::Display>::fmt(self, f)
    }
}

impl super::hash::Hash for BWSSha256Hash {}

// Wrapper for the sha256 Hashing functionalities.
#[derive(Clone, Debug, Default)]
pub struct BWSSha256Hasher {
    state: sha2::Sha256,
}

impl BWSSha256Hasher {
    pub fn new() -> Self {
        Self {
            state: sha2::Sha256::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    pub fn finalize(self) -> BWSSha256Hash {
        BWSSha256Hash(self.state.finalize().into())
    }

    pub fn concat_and_hash(v1: &BWSSha256Hash, v2: &BWSSha256Hash) -> BWSSha256Hash {
        let mut hasher = Self::new();
        hasher.update(v1.as_ref());
        hasher.update(v2.as_ref());
        hasher.finalize()
    }

    pub fn hash(data: &[u8]) -> BWSSha256Hash {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}

#[cfg(test)]
impl BWSSha256Hasher {
    fn finalize_reset(&mut self) -> BWSSha256Hash {
        BWSSha256Hash(self.state.finalize_reset().into())
    }
}

#[cfg(test)]
mod tests {
    use super::BWSSha256Hasher;
    use crate::core::vcs::bws_sha256_hash;

    #[test]
    fn single_hash_test() {
        let hash_a = bws_sha256_hash::BWSSha256Hasher::hash(b"a");
        assert_eq!(
            hash_a.to_string(),
            "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
        );
    }

    #[test]
    fn hash_state_test() {
        let mut state = BWSSha256Hasher::new();
        state.update(b"a");
        state.update(b"b");
        let hash = state.finalize_reset();
        let hash_empty = state.finalize();

        assert_eq!(hash.to_string(), BWSSha256Hasher::hash(b"ab").to_string());
        assert_eq!(
            hash_empty.to_string(),
            BWSSha256Hasher::hash(b"").to_string()
        );
    }
}
