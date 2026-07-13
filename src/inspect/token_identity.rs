//! Classifies filename tokens as delimiters or words.

/// TokenIdentity categorizes the currently identified state of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenIdentity {
    /// A non-alphanumeric character.
    Delimiter,
    /// A tagged token.
    Word,
}

impl TokenIdentity {
    /// Returns the string representation of the token identity.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Delimiter => "delimiter",
            Self::Word => "word",
        }
    }
}
