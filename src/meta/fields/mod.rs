//! Defines normalized media metadata field values.

mod air_date;
mod audio_codec;
mod audio_layout;
mod audio_profile;
mod country;
mod language;
mod media_format;
mod media_type;
mod release_source;
mod track;
mod video_codec;
mod video_dynamic_range;
mod video_profile;
mod video_resolution;

pub use air_date::AirDate;
pub use audio_codec::AudioCodec;
pub use audio_layout::AudioLayout;
pub use audio_profile::AudioProfile;
pub use language::{LANG_ALL, Language};
pub use media_format::{ContentKind, MediaFormat};
pub use media_type::MediaType;
pub use release_source::ReleaseSource;
pub use track::{TrackDisposition, TrackKind, TrackMetadata};
pub use video_codec::VideoCodec;
pub use video_dynamic_range::VideoDynamicRange;
pub use video_profile::VideoProfile;
pub use video_resolution::VideoResolution;

#[expect(
    dead_code,
    reason = "field abstraction is reserved for metadata formatting"
)]
trait Field {
    fn label(&self) -> &'static str;
    fn key(&self) -> &'static str;
    fn value(&self) -> String;
}
