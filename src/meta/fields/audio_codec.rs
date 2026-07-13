//! Defines normalized audio codec values and aliases.

crate::macros::convertable_enum! {
    /// Audio codecs supported by the library.
    AudioCodec,
    /// Advanced Audio Coding.
    Aac => "aac",
    /// Apple Lossless Audio Codec.
    Alac => "alac",
    /// Dolby Atmos spatial audio.
    DolbyAtmos => "dolby_atmos",
    /// Dolby Digital (AC-3).
    DolbyDigital => "dolby_digital",
    /// Dolby Digital Plus (E-AC-3).
    DolbyDigitalPlus => "dolby_digital_plus",
    /// Dolby TrueHD lossless audio.
    DolbyTrueHD => "dolby_true_hd",
    /// DTS core audio.
    Dts => "dts",
    /// DTS-HD Master Audio.
    DtsHD => "dts_hd",
    /// DTS:X object-based audio.
    DtsX => "dts_x",
    /// Free Lossless Audio Codec.
    Flac => "flac",
    /// Linear Pulse-Code Modulation.
    Lpcm => "lpcm",
    /// MPEG-1 Audio Layer III.
    Mp3 => "mp3",
    /// Opus interactive audio codec.
    Opus => "opus",
    /// Pulse-Code Modulation.
    Pcm => "pcm",
    /// Ogg Vorbis.
    Vorbis => "vorbis",
}

crate::unit_tests!("audio_codec.test.rs");
