//! Defines media formats, extensions, and MIME types.

/// The broad kind of content stored by a media file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentKind {
    /// A format that can carry audio and/or video content.
    AudioVideo,
    /// A format containing an external subtitle track.
    Subtitle,
}

/// A media file format recognized from a filename extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum MediaFormat {
    /// 3GPP multimedia.
    #[cfg_attr(feature = "serde", serde(rename = "3gp"))]
    ThreeGp,
    /// Advanced SubStation Alpha subtitles.
    Ass,
    /// Audio Video Interleave.
    Avi,
    /// VobSub index.
    Idx,
    /// MPEG-4 video.
    M4v,
    /// Blu-ray MPEG transport stream.
    M2ts,
    /// Matroska multimedia.
    Mkv,
    /// Multiple-image Network Graphics.
    Mng,
    /// QuickTime multimedia.
    Mov,
    /// MPEG-4 multimedia.
    Mp4,
    /// MPEG program stream.
    Mpeg,
    /// AVCHD MPEG transport stream.
    Mts,
    /// Ogg multimedia.
    Ogg,
    /// Ogg video.
    Ogv,
    /// SubRip subtitles.
    Srt,
    /// SubStation Alpha subtitles.
    Ssa,
    /// VobSub data or MicroDVD subtitles.
    Sub,
    /// MPEG transport stream.
    Ts,
    /// DVD video object.
    Vob,
    /// WebVTT subtitles.
    Vtt,
    /// WebM multimedia.
    Webm,
    /// Windows Media Video.
    Wmv,
}

impl MediaFormat {
    /// Every media file format recognized by extension.
    pub const ALL: [Self; 22] = [
        Self::ThreeGp,
        Self::Ass,
        Self::Avi,
        Self::Idx,
        Self::M4v,
        Self::M2ts,
        Self::Mkv,
        Self::Mng,
        Self::Mov,
        Self::Mp4,
        Self::Mpeg,
        Self::Mts,
        Self::Ogg,
        Self::Ogv,
        Self::Srt,
        Self::Ssa,
        Self::Sub,
        Self::Ts,
        Self::Vob,
        Self::Vtt,
        Self::Webm,
        Self::Wmv,
    ];

    /// Parses a filename extension with or without a leading dot.
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension
            .trim_start_matches('.')
            .to_ascii_lowercase()
            .as_str()
        {
            "3gp" => Some(Self::ThreeGp),
            "ass" => Some(Self::Ass),
            "avi" => Some(Self::Avi),
            "idx" => Some(Self::Idx),
            "m4v" => Some(Self::M4v),
            "m2ts" => Some(Self::M2ts),
            "mkv" => Some(Self::Mkv),
            "mng" => Some(Self::Mng),
            "mov" => Some(Self::Mov),
            "mp4" => Some(Self::Mp4),
            "mpg" | "mpeg" => Some(Self::Mpeg),
            "mts" => Some(Self::Mts),
            "ogg" => Some(Self::Ogg),
            "ogv" => Some(Self::Ogv),
            "srt" => Some(Self::Srt),
            "ssa" => Some(Self::Ssa),
            "sub" => Some(Self::Sub),
            "ts" => Some(Self::Ts),
            "vob" => Some(Self::Vob),
            "vtt" => Some(Self::Vtt),
            "webm" => Some(Self::Webm),
            "wmv" => Some(Self::Wmv),
            _ => None,
        }
    }

    /// Returns the canonical filename extension without a leading dot.
    pub const fn extension(self) -> &'static str {
        match self {
            Self::ThreeGp => "3gp",
            Self::Ass => "ass",
            Self::Avi => "avi",
            Self::Idx => "idx",
            Self::M4v => "m4v",
            Self::M2ts => "m2ts",
            Self::Mkv => "mkv",
            Self::Mng => "mng",
            Self::Mov => "mov",
            Self::Mp4 => "mp4",
            Self::Mpeg => "mpeg",
            Self::Mts => "mts",
            Self::Ogg => "ogg",
            Self::Ogv => "ogv",
            Self::Srt => "srt",
            Self::Ssa => "ssa",
            Self::Sub => "sub",
            Self::Ts => "ts",
            Self::Vob => "vob",
            Self::Vtt => "vtt",
            Self::Webm => "webm",
            Self::Wmv => "wmv",
        }
    }

    /// Returns the MIME type normally associated with the format.
    pub const fn mime_type(self) -> &'static str {
        match self {
            Self::ThreeGp => "video/3gpp",
            Self::Ass | Self::Ssa => "text/x-ssa",
            Self::Avi => "video/x-msvideo",
            Self::Idx => "application/x-vobsub",
            Self::M4v => "video/x-m4v",
            Self::M2ts | Self::Mts | Self::Ts => "video/mp2t",
            Self::Mkv => "video/x-matroska",
            Self::Mng => "video/x-mng",
            Self::Mov => "video/quicktime",
            Self::Mp4 => "video/mp4",
            Self::Mpeg => "video/mpeg",
            Self::Ogg | Self::Ogv => "video/ogg",
            Self::Srt => "application/x-subrip",
            Self::Sub => "text/x-microdvd",
            Self::Vob => "video/dvd",
            Self::Vtt => "text/vtt",
            Self::Webm => "video/webm",
            Self::Wmv => "video/x-ms-wmv",
        }
    }

    /// Returns the broad content kind carried by the format.
    pub const fn content_kind(self) -> ContentKind {
        match self {
            Self::Ass | Self::Idx | Self::Srt | Self::Ssa | Self::Sub | Self::Vtt => {
                ContentKind::Subtitle
            }
            _ => ContentKind::AudioVideo,
        }
    }

    /// Returns whether this is an external subtitle format.
    pub const fn is_subtitle(self) -> bool {
        matches!(self.content_kind(), ContentKind::Subtitle)
    }
}

impl std::fmt::Display for MediaFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.extension())
    }
}

crate::unit_tests!("media_format.test.rs");
