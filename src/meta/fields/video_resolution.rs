//! Defines normalized video resolution values.

crate::macros::convertable_enum! {
    /// Video resolutions recognized by the library.
    VideoResolution,
    /// 360i interlaced.
    Sd360i => "360i",
    /// 360p progressive.
    Sd360p => "360p",
    /// 480i interlaced (SD).
    Sd480i => "480i",
    /// 480p progressive (SD).
    Sd480P => "480p",
    /// 1080i interlaced (Full HD).
    Hd1080i => "1080i",
    /// 1080p progressive (Full HD).
    Hd1080p => "1080p",
    /// 720i interlaced (HD).
    Hd720i => "720i",
    /// 720p progressive (HD).
    Hd720p => "720p",
    /// 4K Ultra HD.
    Uhd4k => "4k",
    /// 8K Ultra HD.
    Uhd8k => "8k",
}
