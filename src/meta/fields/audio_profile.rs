//! Defines normalized audio codec-profile values.

crate::macros::convertable_enum! {
    /// Audio encoding profiles recognized by the library.
    AudioProfile,
    /// DTS-HD Master Audio.
    MasterAudio => "master_audio",
    /// DTS-HD High Resolution Audio.
    HighResolutionAudio => "high_resolution_audio",
    /// DTS Extended Surround.
    ExtendedSurround => "extended_surround",
    /// High Efficiency AAC.
    HighEfficiency => "high_efficiency",
    /// Low Complexity AAC.
    LowComplexity => "low_complexity",
    /// Dolby Digital High Quality.
    HighQuality => "high_quality",
    /// Dolby Digital EX.
    Ex => "ex",
}
