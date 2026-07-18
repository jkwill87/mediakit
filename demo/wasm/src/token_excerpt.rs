//! Defines compact serializable filename-token excerpts.

use mediakit::inspect::Token;
use serde::Serialize;

#[derive(Serialize)]
#[expect(dead_code, reason = "reserved for a future compact WASM response")]
pub struct TokenExcerpt {
    start: i32,
    end: i32,
    key: &'static str,
    value: Option<String>,
}

impl TryFrom<&Token> for TokenExcerpt {
    type Error = ();

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let (key, value) = if let Some(tag) = token.tag.as_ref() {
            (tag.key(), Some(tag.value()))
        } else {
            (token.ident.as_str(), None)
        };
        Ok(Self {
            start: token.start as i32,
            end: token.end as i32,
            key,
            value,
        })
    }
}
