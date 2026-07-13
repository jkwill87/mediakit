//! Inspects filename tokens for audio channel-layout metadata.

use super::{CASE_INSENSITIVE, EOL};
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::macros::recat;
use crate::meta::Tag;
use crate::meta::fields::AudioLayout;
use crate::regexp::RegexVar;
use const_format::concatcp;

const LAYOUT_BOL: &str = r"(?:^|[._ -]|[a-zA-Z])";

const FULL_PARTIAL: &str = r"(?<full>[57])\.(?<sub>\d)(?:\.(?<height>\d))?";
const MONO_PARTIAL: &str = r"(?<mono>1ch|mono)";
const STEREO_PARTIAL: &str = r"(?<stereo>2ch|stereo)";
const SURROUND_5_1_PARTIAL: &str = r"(?<surround_5_1>(?:5\.1|5|6)(?:ch))";
const SURROUND_7_1_PARTIAL: &str = r"(?<surround_7_1>(?:7\.1|7|8)(?:ch))";

impl FilenameInspector {
    /// Selects the audio layout for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_audio_layout(self) -> Self {
        let mut re = RegexVar::new(concatcp!(
            CASE_INSENSITIVE,
            LAYOUT_BOL,
            recat!(
                FULL_PARTIAL,
                MONO_PARTIAL,
                STEREO_PARTIAL,
                SURROUND_5_1_PARTIAL,
                SURROUND_7_1_PARTIAL
            ),
            EOL
        ));
        let audio_layout;
        let start;
        let end;

        if !re.search(&self.filename) {
            return self;
        }

        let mut tokens = self.tokens;

        if let Some(full) = re.get("full") {
            let maybe_height = re.get("height");
            let sub = re.get("sub").unwrap();
            audio_layout = AudioLayout {
                full: full.template(&self.filename).parse::<u8>().unwrap(),
                sub: sub.template(&self.filename).parse::<u8>().unwrap(),
                height: maybe_height
                    .as_ref()
                    .map_or(0, |m| m.template(&self.filename).parse::<u8>().unwrap_or(0)),
            };
            start = full.start;
            end = maybe_height.map_or(sub.end, |m| m.end);
        } else if let Some(mono) = re.get("mono") {
            audio_layout = AudioLayout {
                full: 1,
                sub: 0,
                height: 0,
            };
            start = mono.start;
            end = mono.end;
        } else if let Some(stereo) = re.get("stereo") {
            audio_layout = AudioLayout {
                full: 2,
                sub: 0,
                height: 0,
            };
            start = stereo.start;
            end = stereo.end;
        } else if let Some(surround_5_1) = re.get("surround_5_1") {
            audio_layout = AudioLayout {
                full: 5,
                sub: 1,
                height: 0,
            };
            start = surround_5_1.start;
            end = surround_5_1.end;
        } else if let Some(surround_7_1) = re.get("surround_7_1") {
            audio_layout = AudioLayout {
                full: 7,
                sub: 1,
                height: 0,
            };
            start = surround_7_1.start;
            end = surround_7_1.end;
        } else {
            panic!("Unexpected audio layout");
        }
        let start_token_idx = tokens.iter().position(|t| t.end > start).unwrap();
        let end_token_idx = tokens
            .iter()
            .rev()
            .position(|t| t.start < end)
            .map(|idx| tokens.len() - idx)
            .unwrap();
        let token = Token {
            start,
            end,
            ident: TokenIdentity::Word,
            tag: Some(Tag::AudioLayout(audio_layout)),
        };
        tokens.splice(start_token_idx..end_token_idx, vec![token]);
        Self { tokens, ..self }
    }
}

crate::unit_tests!("audio_layout.test.rs");
