//! Mint Information
// https://github.com/cashubtc/nuts/blob/main/09.md

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::nut01::PublicKey;

/// Mint Version
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintVersion {
    pub name: String,
    pub version: String,
}

impl Serialize for MintVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let combined = format!("{}/{}", self.name, self.version);
        serializer.serialize_str(&combined)
    }
}

impl<'de> Deserialize<'de> for MintVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let combined = String::deserialize(deserializer)?;
        let parts: Vec<&str> = combined.split('/').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid input string"));
        }
        Ok(MintVersion {
            name: parts[0].to_string(),
            version: parts[1].to_string(),
        })
    }
}

/// Mint Info [NIP-09]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintInfo {
    /// name of the mint and should be recognizable
    pub name: Option<String>,
    /// hex pubkey of the mint
    pub pubkey: Option<PublicKey>,
    /// implementation name and the version running
    pub version: Option<MintVersion>,
    /// short description of the mint
    pub description: Option<String>,
    /// long description
    pub description_long: Option<String>,
    /// contact methods to reach the mint operator
    pub contact: Vec<Vec<String>>,
    /// shows which NUTs the mint supports
    pub nuts: Vec<String>,
    /// message of the day that the wallet must display to the user
    pub motd: Option<String>,
}
