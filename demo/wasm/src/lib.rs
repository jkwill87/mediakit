//! Implements WebAssembly bindings for media filename inspection.

use mediakit::inspect::Token;
use mediakit::inspect::{FilenameInspector, Inspector};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

mod token_excerpt;

#[wasm_bindgen(js_name = "callOnce")]
pub fn call_once() {
    console_error_panic_hook::set_once();
}

#[derive(Tsify, Serialize)]
#[serde(rename_all = "lowercase")]
enum InspectStatus {
    Matched,
    Unmatched,
    Delimiter,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "InspectResult")]
    pub type InsepctResult;
}

#[derive(Tsify, Serialize)]
pub struct TokenExcerpt {
    start: i32,
    end: i32,
    status: InspectStatus,
    key: &'static str,
    value: Option<String>,
}

#[derive(Tsify, Serialize)]
pub struct MetadataField {
    key: &'static str,
    value: String,
}

impl TokenExcerpt {
    const fn delimiter(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            status: InspectStatus::Delimiter,
            key: "delimiter",
            value: None,
        }
    }

    const fn unmatched(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            status: InspectStatus::Unmatched,
            key: "word",
            value: None,
        }
    }
}

impl TryFrom<Token> for TokenExcerpt {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let (key, value) = if let Some(tag) = token.tag {
            (tag.key(), Some(tag.value()))
        } else {
            (token.ident.as_str(), None)
        };
        let status = match value {
            Some(_) => InspectStatus::Matched,
            None => {
                if key == "delimiter" {
                    InspectStatus::Delimiter
                } else {
                    InspectStatus::Unmatched
                }
            }
        };
        Ok(Self {
            start: token.start as i32,
            end: token.end as i32,
            key,
            value,
            status,
        })
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct InspectResult {
    pub tokens: Vec<TokenExcerpt>,
    pub metadata: Vec<MetadataField>,
    pub media_type: String,
}

#[wasm_bindgen(js_name = "inspectFilepath")]
pub fn inspect_filepath(path: &str) -> InspectResult {
    let filename_inspector = FilenameInspector::new(path).analyze();
    let prefix_len = path.len() - filename_inspector.filename.len();
    let media_type = filename_inspector.media_type.to_string();
    let mut metadata = vec![MetadataField {
        key: "media_type",
        value: media_type.clone(),
    }];
    metadata.extend(
        filename_inspector
            .tags()
            .into_iter()
            .map(|tag| MetadataField {
                key: tag.key(),
                value: tag.value(),
            }),
    );

    // Tokenize the path prefix (directory components before the filename)
    let mut tokens = Vec::new();
    let prefix = &path[..prefix_len];
    let mut start_idx = 0;
    for (idx, c) in prefix.char_indices() {
        if c.is_whitespace() || !c.is_alphanumeric() {
            if start_idx < idx {
                tokens.push(TokenExcerpt::unmatched(start_idx as i32, idx as i32));
            }
            tokens.push(TokenExcerpt::delimiter(
                idx as i32,
                (idx + c.len_utf8()) as i32,
            ));
            start_idx = idx + c.len_utf8();
        }
    }
    if start_idx < prefix.len() {
        tokens.push(TokenExcerpt::unmatched(
            start_idx as i32,
            prefix.len() as i32,
        ));
    }

    // Append filename tokens, offset by the prefix length
    let prefix_len = prefix_len as i32;
    tokens.extend(filename_inspector.tokens.into_iter().filter_map(|token| {
        TokenExcerpt::try_from(token).ok().map(|mut t| {
            t.start += prefix_len;
            t.end += prefix_len;
            t
        })
    }));

    InspectResult {
        tokens,
        metadata,
        media_type,
    }
}
