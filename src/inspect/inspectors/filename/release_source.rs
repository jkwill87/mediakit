//! Inspects filename tokens for release sources.

use super::{BOL, CASE_INSENSITIVE, EOL};
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::ReleaseSource;
use crate::regexp::RegexVar;
use const_format::concatcp;
use std::str::FromStr;

impl FilenameInspector {
    /// Selects the release source for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_release_source(self) -> Self {
        let mut re = RegexVar::new(concatcp!(
            CASE_INSENSITIVE,
            BOL,
            r"(?:",
            // Blu-ray
            r"(?<bluray>(?:bluray|blu|bd|br)(?:[._ -]?rip)?)",
            // DVD
            r"|(?<dvd>((?:dvd)(?:[._ -]?rip)?))",
            // HDTV
            r"|(?<hdtv>(?:hdtv)(?:[._ -]?rip)?)",
            // TELECiNE
            r"|(?<telecine>telecine|tc)",
            // WEB-RIP
            r"|(?<webrip>(web[._ -]?rip))",
            // WEB-DL
            r"|(?<webdl>(web(?:[._ -]?dl)?))",
            r")",
            EOL,
        ));
        if !re.search(&self.filename) {
            return self;
        }

        let mut tokens = self.tokens;

        for regex_match in re.labeled_captures() {
            let value = regex_match.label.unwrap();
            let release_source = ReleaseSource::from_str(value).unwrap();
            let start = regex_match.start;
            let end = regex_match.end;
            let token = Token {
                start,
                end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::ReleaseSource(release_source)),
            };
            let start_token_idx = tokens.iter().position(|t| t.end > start).unwrap();
            let end_token_idx = tokens
                .iter()
                .rev()
                .position(|t| t.start < end)
                .map(|idx| tokens.len() - idx)
                .unwrap();
            if tokens[start_token_idx..end_token_idx]
                .iter()
                .any(|token| token.tag.is_some())
            {
                continue;
            }
            tokens.splice(start_token_idx..end_token_idx, vec![token]);
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("release_source.test.rs");
