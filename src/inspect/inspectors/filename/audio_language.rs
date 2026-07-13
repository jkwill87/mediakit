//! Inspects filename tokens for audio language metadata.

use crate::inspect::{FilenameInspector, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::Language;

impl FilenameInspector {
    /// Selects encoded audio-language and subtitle-language markers.
    ///
    /// External-track language suffixes are handled by the track-suffix
    /// inspector. Audio languages are recognized after a structural or
    /// technical metadata marker, which avoids treating ordinary title words
    /// as language codes.
    ///
    /// **Preconditions:**
    /// - Requires the file format and external-track suffixes to have been inspected.
    /// - Requires structural and technical metadata to have been previously selected for audio languages.
    pub(super) fn inspect_audio_language(mut self) -> Self {
        if self
            .metadata
            .format
            .is_some_and(|format| format.is_subtitle())
        {
            return self;
        }

        let Some(marker_idx) = self.tokens.iter().position(|token| {
            matches!(
                token.tag,
                Some(Tag::AirDate(_))
                    | Some(Tag::SeasonNumber(_))
                    | Some(Tag::EpisodeNumber(_))
                    | Some(Tag::PremiereYear(_))
                    | Some(Tag::ReleaseSource(_))
                    | Some(Tag::AudioCodec(_))
                    | Some(Tag::AudioProfile(_))
                    | Some(Tag::AudioLayout(_))
                    | Some(Tag::VideoCodec(_))
                    | Some(Tag::VideoProfile(_))
                    | Some(Tag::VideoDynamicRange(_))
                    | Some(Tag::VideoResolution(_))
            )
        }) else {
            return self;
        };

        for token in self.tokens.iter_mut().skip(marker_idx + 1) {
            if token.tag.is_some() || !matches!(token.ident, TokenIdentity::Word) {
                continue;
            }
            if let Some(language) = Language::from_identifier(token.template(&self.filename)) {
                token.tag = Some(Tag::AudioLanguage(language));
            }
        }
        self
    }
}

crate::unit_tests!("audio_language.test.rs");
