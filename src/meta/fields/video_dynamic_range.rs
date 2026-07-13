//! Defines normalized video dynamic-range values.

crate::macros::convertable_enum!(
    /// Video dynamic ranges recognized by the library.
    VideoDynamicRange,
    /// Standard Dynamic Range.
    SDR => "sdr",
    /// HDR10 (10-bit).
    HDR10 => "hdr10",
    /// HDR12 (12-bit).
    HDR12 => "hdr12",
    /// Dolby Vision.
    DolbyVision => "dolby_vision",
);
