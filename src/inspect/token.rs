//! Represents positioned filename tokens and their parsed metadata.

use crate::inspect::token_identity::TokenIdentity;
use crate::meta::Tag;

/// Token tracks the position and identity of a token in a string.
pub struct Token {
    /// The inclusive starting byte index of the token in the original string.
    pub start: usize,
    /// The exclusive end byte index of the token in the original string.
    pub end: usize,
    /// The token identity.
    pub ident: TokenIdentity,
    /// Metadata associated with a parsed token. Always `Some` for known tokens, `None` otherwise.
    pub tag: Option<Tag>,
}

impl Token {
    pub(super) fn template<'a>(&self, s: &'a str) -> &'a str {
        s[self.start..self.end].trim()
    }
}
