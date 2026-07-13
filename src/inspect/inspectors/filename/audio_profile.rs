//! Inspects filename tokens for audio codec-profile metadata.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::{AudioCodec, AudioProfile};

impl FilenameInspector {
    /// Selects audio encoding profiles associated with a detected audio codec.
    ///
    /// **Preconditions:**
    /// - Requires the audio codec to have been previously selected.
    pub(super) fn inspect_audio_profile(mut self) -> Self {
        for idx in 0..self.tokens.len() {
            let Some(Tag::AudioCodec(codec)) = self.tokens[idx].tag.as_ref() else {
                continue;
            };
            let lower = self.tokens[idx]
                .template(&self.filename)
                .to_ascii_lowercase();
            let inline_profile = match codec {
                AudioCodec::Aac if lower.ends_with("he") => Some(AudioProfile::HighEfficiency),
                AudioCodec::Aac if lower.ends_with("lc") => Some(AudioProfile::LowComplexity),
                AudioCodec::DolbyDigital if lower.ends_with("hq") => {
                    Some(AudioProfile::HighQuality)
                }
                AudioCodec::DolbyDigital if lower.ends_with("ex") => Some(AudioProfile::Ex),
                _ => None,
            };
            let Some(profile) = inline_profile else {
                continue;
            };
            let profile_start = self.tokens[idx].end - 2;
            let mut codec_end = profile_start;
            while codec_end > self.tokens[idx].start
                && !self.filename.as_bytes()[codec_end - 1].is_ascii_alphanumeric()
            {
                codec_end -= 1;
            }
            let mut replacement = vec![Token {
                start: self.tokens[idx].start,
                end: codec_end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::AudioCodec(codec.clone())),
            }];
            if codec_end < profile_start {
                replacement.push(Token {
                    start: codec_end,
                    end: profile_start,
                    ident: TokenIdentity::Delimiter,
                    tag: None,
                });
            }
            replacement.push(Token {
                start: profile_start,
                end: self.tokens[idx].end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::AudioProfile(profile)),
            });
            self.tokens.splice(idx..=idx, replacement);
            break;
        }

        let codec = self
            .tokens
            .iter()
            .find_map(|token| match token.tag.as_ref() {
                Some(Tag::AudioCodec(codec)) => Some(codec.clone()),
                _ => None,
            });
        let Some(codec) = codec else {
            return self;
        };

        for token in &mut self.tokens {
            if token.tag.is_some() {
                continue;
            }
            let identifier = token.template(&self.filename).to_ascii_lowercase();
            let profile = match (&codec, identifier.as_str()) {
                (AudioCodec::DtsHD, "ma") => AudioProfile::MasterAudio,
                (AudioCodec::DtsHD, "hr" | "hra") => AudioProfile::HighResolutionAudio,
                (AudioCodec::Dts, "es") => AudioProfile::ExtendedSurround,
                (AudioCodec::Aac, "he") => AudioProfile::HighEfficiency,
                (AudioCodec::Aac, "lc") => AudioProfile::LowComplexity,
                (AudioCodec::DolbyDigital, "hq") => AudioProfile::HighQuality,
                (AudioCodec::DolbyDigital, "ex") => AudioProfile::Ex,
                _ => continue,
            };
            token.tag = Some(Tag::AudioProfile(profile));
        }
        self
    }
}

crate::unit_tests!("audio_profile.test.rs");
