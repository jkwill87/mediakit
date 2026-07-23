//! Typed metadata for streams discovered in media containers.
//!
//! Every stream includes [`StreamInfo`] for container-independent flags and language metadata,
//! then adds fields appropriate to its media kind.

mod audio_stream;
mod stream_info;
mod subtitle_stream;
mod video_stream;

pub use audio_stream::AudioStream;
pub use stream_info::StreamInfo;
pub use subtitle_stream::SubtitleStream;
pub use video_stream::VideoStream;

crate::unit_tests!("mod.test.rs");
