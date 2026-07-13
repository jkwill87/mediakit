//! Defines normalized video codec-profile values.

crate::macros::convertable_enum! {
    /// Video encoding profiles recognized by the library.
    VideoProfile,
    /// Baseline profile.
    Baseline => "baseline",
    /// Extended profile.
    Extended => "extended",
    /// Main profile.
    Main => "main",
    /// Main 10 profile.
    Main10 => "main_10",
    /// High profile.
    High => "high",
    /// High 10 profile.
    High10 => "high_10",
    /// High 4:2:2 profile.
    High422 => "high_4_2_2",
    /// High 4:4:4 Predictive profile.
    High444Predictive => "high_4_4_4_predictive",
    /// Scalable Video Coding profile.
    ScalableVideoCoding => "scalable_video_coding",
    /// Advanced Video Coding High Definition profile.
    Avchd => "avchd",
}
