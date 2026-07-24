//! Coordinates ordered filename-token inspection and metadata extraction.

mod alternative_title;
mod audio_codec;
mod audio_layout;
mod audio_profile;
mod episode_title;
mod file_format;
mod language;
mod media_type;
mod premiere_year;
mod release_group;
mod release_source;
mod subtitle_disposition;
mod subtitle_suffix_parser;
mod television_airdate;
mod television_ordering;
mod title;
mod video_codec;
mod video_dynamic_range;
mod video_profile;
mod video_resolution;

use crate::inspect::{Inspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::{MediaFormat, MediaType};
use std::path::PathBuf;

const CASE_INSENSITIVE: &str = r"(?i)";
const BOL: &str = r"(?:^|[._ -])";
const EOL: &str = r"(?:$|[._ -])";

/// Parses media metadata from a file's name.
///
/// Currently able to parse the following metadata [`Tag`]s:
/// - [`Tag::FileFormat`]
/// - [`Tag::SeasonNumber`]
/// - [`Tag::EpisodeNumber`]
/// - [`Tag::AirDate`]
/// - [`Tag::Title`]
/// - [`Tag::AlternativeTitle`]
/// - [`Tag::PremiereYear`]
/// - [`Tag::ReleaseSource`]
/// - [`Tag::ReleaseGroup`]
/// - [`Tag::EpisodeTitle`]
/// - [`Tag::AudioCodec`]
/// - [`Tag::AudioProfile`]
/// - [`Tag::AudioLayout`]
/// - [`Tag::AudioLanguage`]
/// - [`Tag::SubtitleLanguage`]
/// - [`Tag::SubtitleDisposition`]
/// - [`Tag::VideoCodec`]
/// - [`Tag::VideoProfile`]
/// - [`Tag::VideoDynamicRange`]
/// - [`Tag::VideoResolution`]
///
/// Filename components that cannot be represented as UTF-8 are retained in the path but are not
/// tokenized or inspected for tags.
pub struct FilenameInspector {
    path: PathBuf,
    filename: String,
    tokens: Vec<Token>,
    media_type_hint: Option<MediaType>,
}

impl FilenameInspector {
    /// Creates a new [`FilenameInspector`] from a given path.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let path = path.into();
        let filename = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_owned();
        // tokenization
        let pathname_token_count = filename.chars().count();
        let mut tokens = Vec::with_capacity(pathname_token_count);
        let mut start_idx = 0;
        for (end_idx, c) in filename.char_indices() {
            if c.is_whitespace() || !c.is_alphanumeric() {
                // we've found a delimiter
                if start_idx < end_idx {
                    // this marks the end of a word, add it to the token list
                    tokens.push(Token {
                        start: start_idx,
                        end: end_idx,
                        ident: TokenIdentity::Word,
                        tag: None,
                    });
                }
                // add the delimiter to the token list
                tokens.push(Token {
                    start: end_idx,
                    end: end_idx + c.len_utf8(),
                    ident: TokenIdentity::Delimiter,
                    tag: None,
                });
                start_idx = end_idx + c.len_utf8();
            }
        }
        // if there any characters after the last delimiter marker, add the word to the token list
        if start_idx < filename.len() {
            tokens.push(Token {
                start: start_idx,
                end: filename.len(),
                ident: TokenIdentity::Word,
                tag: None,
            });
        }
        Self {
            path,
            filename,
            tokens,
            media_type_hint: None,
        }
    }

    /// Returns the path string that was parsed.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Returns the tokens parsed from the path string.
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    /// Returns the suffix-free identity stem, when it can be represented as UTF-8.
    pub fn identity_stem(&self) -> Option<&str> {
        subtitle_suffix_parser::ParsedSubtitleSuffix::parse(&self.path).map_or_else(
            || self.path.file_stem().and_then(|stem| stem.to_str()),
            |parsed| parsed.identity_stem(),
        )
    }

    /// Supplies an explicit media type hint for ambiguous filenames.
    ///
    /// A movie or television hint overrides automatic media-type detection while leaving the rest
    /// of the inspection pipeline unchanged. Passing [`MediaType::Unknown`] restores automatic
    /// detection.
    pub fn with_media_type_hint(mut self, media_type: MediaType) -> Self {
        self.media_type_hint = (media_type != MediaType::Unknown).then_some(media_type);
        self
    }
}

impl Inspector for FilenameInspector {
    fn analyze(self) -> Self {
        self //
            .inspect_file_format()
            .inspect_subtitle_disposition()
            .inspect_leading_release_group()
            .inspect_television_ordering()
            .inspect_television_air_date()
            .inspect_release_source()
            .inspect_audio_codec()
            .inspect_audio_profile()
            .inspect_audio_layout()
            .inspect_video_codec()
            .inspect_video_profile()
            .inspect_video_dynamic_range()
            .inpsect_video_resolution()
            .inspect_language()
            .inspect_title()
            .inspect_premiere_year()
            .inspect_alternative_title()
            .inspect_episode_title()
            .inspect_release_group()
    }

    fn tags(&self) -> Vec<&Tag> {
        self.tokens
            .iter()
            .filter_map(|token| token.tag.as_ref())
            .collect()
    }
}

impl FilenameInspector {
    /// Returns the media format inferred directly from the path extension.
    pub(super) fn file_format(&self) -> Option<MediaFormat> {
        self.path
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(MediaFormat::from_extension)
    }

    /// Returns the end of the subtitle identity before suffix markers.
    pub(super) fn subtitle_identity_end(&self) -> Option<usize> {
        subtitle_suffix_parser::ParsedSubtitleSuffix::parse(&self.path)
            .map(|parsed| parsed.suffix_start())
    }
}

impl std::fmt::Debug for FilenameInspector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct TokenTemplate<'a> {
            token: &'a Token,
            template: &'a str,
        }
        impl std::fmt::Debug for TokenTemplate<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Token")
                    .field("start", &self.token.start)
                    .field("end", &self.token.end)
                    .field("ident", &self.token.ident)
                    .field("text", &self.template)
                    .field("tag", &self.token.tag)
                    .finish()
            }
        }
        let token_templates = self
            .tokens
            .iter()
            .map(|token| TokenTemplate {
                token,
                template: token.template(&self.filename),
            })
            .collect::<Vec<_>>();
        f.debug_struct("FilenameInspector")
            .field("pathname", &self.filename)
            .field("tokens", &token_templates)
            .finish()
    }
}

crate::unit_tests!("mod.test.rs");
