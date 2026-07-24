//! Typed metadata for tracks discovered in media containers.
//!
//! Every track includes [`TrackInfo`] for container-independent flags and language metadata, then
//! adds fields appropriate to its media kind. [`Track`] preserves the native cross-kind order.

mod audio_track;
mod subtitle_codec;
mod subtitle_track;
mod track;
mod track_info;
mod video_track;

pub use audio_track::AudioTrack;
pub use subtitle_codec::SubtitleCodec;
pub use subtitle_track::SubtitleTrack;
pub use track::Track;
pub use track_info::TrackInfo;
pub use video_track::VideoTrack;

crate::unit_tests!("mod.test.rs");
