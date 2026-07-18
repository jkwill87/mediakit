//! Defines tagged metadata values produced by inspection.

use super::fields;
use std::fmt::Debug;

/// Metadata extracted from a media file or its filename.
#[derive(Debug)]
pub enum Tag {
    /// The media file format inferred from the filename extension.
    FileFormat(fields::MediaFormat),

    /// The container format detected from the media file's content.
    Container(fields::MediaFormat),

    /// The MIME type of the media file, e.g. `video/x-matroska`, `video/mp4`, `video/x-msvideo`.
    MimeType(String),

    /// The size of the media file (in bytes).
    FileSize(u64),

    /// The title of the movie or television series.
    Title(String),

    /// An alternate or secondary title fragment.
    AlternativeTitle(String),

    /// The air date of the movie or television series.
    AirDate(fields::AirDate),

    /// The release group responsible for publishing or distributing the media file.
    ReleaseGroup(String),

    /// The source medium of the media file.
    ReleaseSource(fields::ReleaseSource),

    /// The year the movie or television series premiered.
    PremiereYear(u16),

    /// The runtime of the movie or television series (in seconds).
    Runtime(u64),

    /// The title of the episode.
    EpisodeTitle(String),

    /// The episode number of the episode.
    EpisodeNumber(u16),

    /// The season number of the episode.
    SeasonNumber(u16),

    /// The audio bit rate of the media file (in bits per second).
    AudioBitRate(u32),

    /// The audio layout of the media file.
    AudioLayout(fields::AudioLayout),

    /// The audio codec of the media file
    AudioCodec(fields::AudioCodec),

    /// The audio encoding profile of the media file.
    AudioProfile(fields::AudioProfile),

    /// The audio language of the media file.
    AudioLanguage(fields::Language),

    /// The language of a subtitle file.
    SubtitleLanguage(fields::Language),

    /// A numeric subtitle track discriminator.
    SubtitleTrack(u16),

    /// A subtitle track disposition encoded in the filename.
    SubtitleDisposition(fields::TrackDisposition),

    /// The video dynamic range of the media file.
    VideoDynamicRange(fields::VideoDynamicRange),

    /// The video codec of the media file.
    VideoCodec(fields::VideoCodec),

    /// The video encoding profile of the media file.
    VideoProfile(fields::VideoProfile),

    /// The video frame rate of the media file (in frames per second).
    VideoFrameRate(f32),

    /// The video resolution of the media file.
    VideoResolution(fields::VideoResolution),

    /// The disc number of the media file.
    DiscNumber(u8),
}

macro_rules! tag_impl {
    ($($variant:ident => $key:expr),* $(,)?) => {
        /// Returns the label of the tag.
        pub const fn key(&self) -> &'static str {
            match self {
                $(Self::$variant(_) => $key,)*
            }
        }

        /// Returns the value of the tag.
        pub fn value(&self) -> String {
            match self {
                $(Self::$variant(value) => value.to_string(),)*
            }
        }
    };
}

impl Tag {
    tag_impl! {
        FileFormat => "file_format",
        Container => "container",
        MimeType => "mime_type",
        FileSize => "file_size",
        Title => "title",
        AlternativeTitle => "alternative_title",
        AirDate => "air_date",
        ReleaseGroup => "release_group",
        ReleaseSource => "release_source",
        PremiereYear => "year",
        Runtime => "runtime",
        EpisodeTitle => "episode_title",
        EpisodeNumber => "episode_number",
        SeasonNumber => "season_number",
        AudioBitRate => "audio_bit_rate",
        AudioLayout => "audio_layout",
        AudioCodec => "audio_codec",
        AudioProfile => "audio_profile",
        AudioLanguage => "audio_language",
        SubtitleLanguage => "subtitle_language",
        SubtitleTrack => "subtitle_track",
        SubtitleDisposition => "subtitle_disposition",
        VideoDynamicRange => "video_dynamic_range",
        VideoCodec => "video_codec",
        VideoProfile => "video_profile",
        VideoFrameRate => "video_frame_rate",
        VideoResolution => "video_resolution",
        DiscNumber => "disc_number",
    }
}
