//! Defines normalized video codec values and aliases.

crate::macros::convertable_enum!(
    /// Video codecs supported by the library.
    VideoCodec,
    /// AOMedia Video 1.
    Av1 => "av1",
    /// H.262 / MPEG-2 Part 2.
    H262 => "h262",
    /// H.264 / AVC.
    H264 => "h264",
    /// H.265 / HEVC.
    H265 => "h265",
    /// MPEG-4 Part 2 Visual.
    Mpeg4Visual => "mpeg4_visual",
    /// VC-1 / WMV3.
    Vc1 => "vc1",
    /// VP8.
    Vp8 => "vp8",
    /// VP9.
    Vp9 => "vp9",
);
