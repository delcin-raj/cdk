//! Request mint
// https://github.com/cashubtc/nuts/blob/main/03.md

use serde::{Deserialize, Serialize};

pub use crate::Invoice;

/// Mint request response [NUT-03]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestMintResponse {
    /// Bolt11 payment request
    pub pr: Invoice,
    /// Random hash MUST not be the hash of invoice
    pub hash: String,
}
