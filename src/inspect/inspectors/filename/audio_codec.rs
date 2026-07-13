//! Inspects filename tokens for audio codec metadata.

use super::{BOL, CASE_INSENSITIVE};
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::macros::recat;
use crate::meta::Tag;
use crate::meta::fields::AudioCodec;
use crate::regexp::RegexVar;
use const_format::concatcp;
use std::str::FromStr;

const CODEC_EOL: &str = r"(?:$|[._ -]|\d)";

const AAC_PARTIAL: &str = r"(?<aac>aac(?:[ .:-](?:he|lc))?)";
const ALAC_PARTIAL: &str = r"(?<alac>alac)";
const DOLBY_ATMOS_PARTIAL: &str = r"(?<dolby_atmos>atmos|dolby[ .-]atmos)";
const DOLBY_DIGITAL_PLUS_PARTIAL: &str = r"(?<dolby_digital_plus>dolby-digital-plus|ddp|e-?ac-?3)";
const DOLBY_TRUE_HD_PARTIAL: &str = r"(?<dolby_true_hd>true-?hd)";
const DOLBY_DIGITAL_PARTIAL: &str =
    r"(?<dolby_digital>dolby-?digital|dolby|dd|ac-?3d?(?:[ .:-](?:hq|ex))?)";
const DTS_HD_PARTIAL: &str = r"(?<dts_hd>dts-?(?:hd|ma))";
const DTS_X_PARTIAL: &str = r"(?<dts_x>dts[:-]?x)";
const DTS_PARTIAL: &str = r"(?<dts>dts)";
const FLAC_PARTIAL: &str = r"(?<flac>flac)";
const MP3_PARTIAL: &str = r"(?<mp3>mp3|lame)";
const OPUS_PARTIAL: &str = r"(?<opus>opus)";
const PCM_PARTIAL: &str = r"(?<pcm>pcm)";
const LPCM_PARTIAL: &str = r"(?<lpcm>lpcm)";
const VORBIS_PARTIAL: &str = r"(?<vorbis>vorbis)";

impl FilenameInspector {
    /// Selects the audio codec for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_audio_codec(self) -> Self {
        // order is relevant; earlier matches will take precedence
        let regexp = concatcp!(
            CASE_INSENSITIVE,
            BOL,
            recat!(
                AAC_PARTIAL,
                ALAC_PARTIAL,
                DOLBY_ATMOS_PARTIAL,
                DOLBY_DIGITAL_PLUS_PARTIAL,
                DOLBY_TRUE_HD_PARTIAL,
                DOLBY_DIGITAL_PARTIAL,
                DTS_HD_PARTIAL,
                DTS_X_PARTIAL,
                DTS_PARTIAL,
                FLAC_PARTIAL,
                MP3_PARTIAL,
                OPUS_PARTIAL,
                PCM_PARTIAL,
                LPCM_PARTIAL,
                VORBIS_PARTIAL
            ),
            CODEC_EOL,
        );
        let mut re = RegexVar::new(regexp);
        if !re.search(&self.filename) {
            return self;
        }

        let mut tokens = self.tokens;

        for regex_match in re.labeled_captures() {
            let value = regex_match.label.unwrap();
            let audio_codec = AudioCodec::from_str(value).unwrap();
            let start = regex_match.start;
            let end = regex_match.end;
            let start_token_idx = tokens.iter().position(|t| t.end > start).unwrap();
            let end_token_idx = tokens
                .iter()
                .rev()
                .position(|t| t.start < end)
                .map(|idx| tokens.len() - idx)
                .unwrap();
            // only match unmatched text spans
            if !tokens[start_token_idx..end_token_idx]
                .iter()
                .all(|t| t.tag.is_none())
            {
                continue;
            }
            let mut updated_tokens = Vec::with_capacity(3);
            // we may need to split tokens
            if tokens[start_token_idx].start < start {
                // split the leading token
                let leading_word = Token {
                    start: tokens[start_token_idx].start,
                    end: start,
                    ident: tokens[start_token_idx].ident,
                    tag: None,
                };
                updated_tokens.push(leading_word);
            }
            let audio_codec = Token {
                start,
                end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::AudioCodec(audio_codec)),
            };
            updated_tokens.push(audio_codec);
            if tokens[end_token_idx - 1].end > end {
                // split the trailing token
                let trailing_word = Token {
                    start: end,
                    end: tokens[end_token_idx - 1].end,
                    ident: tokens[end_token_idx - 1].ident,
                    tag: None,
                };
                updated_tokens.push(trailing_word);
            }
            // replace the original tokens with the updated tokens
            tokens.splice(start_token_idx..end_token_idx, updated_tokens);
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("audio_codec.test.rs");
