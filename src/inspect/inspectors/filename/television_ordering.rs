//! Inspects filename tokens for season and episode ordering.

use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::regexp::RegexVar;

impl FilenameInspector {
    /// Selects the season and episode number for a television series.
    ///
    /// Recognizes `S01E01` and `01x01` formatting, including multipart episodes such as
    /// `S01E01E02`, `S01E01-E03`, and `S01E01-02`.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_television_ordering(self) -> Self {
        let mut re =
            RegexVar::new(r"(?i)\b(?:(S0*(\d{1,4}))(EP?0*(\d{1,4}))|(0*(\d{1,4}))(x0*(\d{1,4})))");

        if !re.search(&self.filename) {
            return self.inspect_separated_television_ordering();
        }

        let mut captures = re.captures().into_iter();
        captures.next(); // skip the full outer match
        let season_match = captures.next().unwrap();
        let season_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();
        let episode_match = captures.next().unwrap();
        let episode_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();

        let season_text = season_match.template(&self.filename);
        if !season_text.starts_with(['S', 's']) && season_value >= 100 && episode_value >= 100 {
            return self.inspect_separated_television_ordering();
        }

        let mut new_tokens = vec![
            Token {
                start: season_match.start,
                end: season_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::SeasonNumber(season_value)),
            },
            Token {
                start: episode_match.start,
                end: episode_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::EpisodeNumber(episode_value)),
            },
        ];

        // Lookahead for continuation episodes (e.g. E02, -E03, -02)
        let bytes = self.filename.as_bytes();
        let mut pos = episode_match.end;
        loop {
            if pos >= bytes.len() {
                break;
            }

            let has_hyphen = bytes[pos] == b'-';
            let after_hyphen = if has_hyphen { pos + 1 } else { pos };
            if after_hyphen >= bytes.len() {
                break;
            }

            let has_e = bytes[after_hyphen] == b'E' || bytes[after_hyphen] == b'e';
            let digit_start = if has_e {
                after_hyphen + 1
            } else {
                after_hyphen
            };

            if !has_hyphen && !has_e {
                break;
            }
            if digit_start >= bytes.len() || !bytes[digit_start].is_ascii_digit() {
                break;
            }

            let mut digit_end = digit_start;
            while digit_end < bytes.len() && bytes[digit_end].is_ascii_digit() {
                digit_end += 1;
            }

            let ep_value: u16 = match self.filename[digit_start..digit_end].parse() {
                Ok(v) => v,
                Err(_) => break,
            };

            if has_hyphen {
                new_tokens.push(Token {
                    start: pos,
                    end: pos + 1,
                    ident: TokenIdentity::Delimiter,
                    tag: None,
                });
            }

            let ep_token_start = if has_e { after_hyphen } else { digit_start };
            new_tokens.push(Token {
                start: ep_token_start,
                end: digit_end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::EpisodeNumber(ep_value)),
            });

            pos = digit_end;
        }

        let start_idx = self
            .tokens
            .iter()
            .position(|t| t.end > season_match.start)
            .unwrap();
        let end_idx = self.tokens.iter().rposition(|t| t.start < pos).unwrap() + 1;
        let mut tokens = self.tokens;
        tokens.splice(start_idx..end_idx, new_tokens);
        Self { tokens, ..self }
    }

    fn inspect_separated_television_ordering(self) -> Self {
        let mut re = RegexVar::new(r"(?i)\b(S0*(\d{1,4}))[._ ()-]+(EP?[._ ()-]*0*(\d{1,4}))\b");
        if !re.search(&self.filename) {
            return self.inspect_explicit_episode_ordering();
        }

        let mut captures = re.captures().into_iter();
        captures.next();
        let season_match = captures.next().unwrap();
        let season_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();
        let episode_match = captures.next().unwrap();
        let episode_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();

        let replacement = [
            Token {
                start: season_match.start,
                end: season_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::SeasonNumber(season_value)),
            },
            Token {
                start: season_match.end,
                end: episode_match.start,
                ident: TokenIdentity::Delimiter,
                tag: None,
            },
            Token {
                start: episode_match.start,
                end: episode_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::EpisodeNumber(episode_value)),
            },
        ];
        self.replace_span(season_match.start, episode_match.end, replacement)
    }

    fn inspect_explicit_episode_ordering(self) -> Self {
        let mut re = RegexVar::new(r"(?i)\b(EP?[._ ()-]*0*(\d{1,4}))\b");
        if !re.search(&self.filename) {
            return self.inspect_bare_episode_ordering();
        }

        let mut captures = re.captures().into_iter();
        captures.next();
        let episode_match = captures.next().unwrap();
        let episode_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();
        self.replace_span(
            episode_match.start,
            episode_match.end,
            [Token {
                start: episode_match.start,
                end: episode_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::EpisodeNumber(episode_value)),
            }],
        )
    }

    fn inspect_bare_episode_ordering(self) -> Self {
        let mut re = RegexVar::new(r"(?:[._ ]-[._ ]+)(0*(\d{1,3}))(?:$|[._ (\[])");
        if !re.search(&self.filename) {
            return self;
        }

        let mut captures = re.captures().into_iter();
        captures.next();
        let episode_match = captures.next().unwrap();
        let episode_value = captures
            .next()
            .unwrap()
            .template(&self.filename)
            .parse::<u16>()
            .unwrap();
        let strong_separators = self.filename[..episode_match.start]
            .char_indices()
            .filter(|(index, character)| {
                if *character != '-' {
                    return false;
                }
                let previous = self.filename[..*index].chars().next_back();
                let next = self.filename[index + 1..].chars().next();
                !previous.is_some_and(char::is_alphanumeric)
                    || !next.is_some_and(char::is_alphanumeric)
            })
            .count();
        if strong_separators < 2 {
            return self;
        }

        self.replace_span(
            episode_match.start,
            episode_match.end,
            [Token {
                start: episode_match.start,
                end: episode_match.end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::EpisodeNumber(episode_value)),
            }],
        )
    }

    fn replace_span<const N: usize>(
        self,
        start: usize,
        end: usize,
        replacement: [Token; N],
    ) -> Self {
        let start_idx = self
            .tokens
            .iter()
            .position(|token| token.end > start)
            .unwrap();
        let end_idx = self
            .tokens
            .iter()
            .rposition(|token| token.start < end)
            .unwrap()
            + 1;
        let mut tokens = self.tokens;
        tokens.splice(start_idx..end_idx, replacement);
        Self { tokens, ..self }
    }
}

crate::unit_tests!("television_ordering.test.rs");
