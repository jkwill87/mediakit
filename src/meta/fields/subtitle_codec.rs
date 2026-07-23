//! Defines normalized subtitle codec values.

crate::macros::convertable_enum! {
    /// Subtitle codecs supported by the library.
    SubtitleCodec,
    /// ARIB STD-B24 subtitles.
    Arib => "arib",
    /// Advanced SubStation Alpha subtitles.
    Ass => "ass",
    /// Bitmap subtitles.
    Bitmap => "bitmap",
    /// CEA-608 closed captions.
    Cea608 => "cea_608",
    /// CEA-708 closed captions.
    Cea708 => "cea_708",
    /// Digital Video Broadcasting subtitles.
    Dvb => "dvb_subtitle",
    /// HDMV text subtitles.
    HdmvText => "hdmv_text",
    /// Karaoke And Text Encapsulation subtitles.
    Kate => "kate",
    /// Presentation Graphic Stream subtitles.
    Pgs => "pgs",
    /// Plain-text subtitles.
    PlainText => "text",
    /// SubRip subtitles.
    Srt => "srt",
    /// SubStation Alpha subtitles.
    Ssa => "ssa",
    /// Generic subtitle graphics.
    SubtitleGraphics => "subtitle_graphics",
    /// Teletext subtitles.
    Teletext => "teletext",
    /// 3GPP timed text subtitles.
    TimedText => "timed_text",
    /// Timed Text Markup Language subtitles.
    Ttml => "ttml",
    /// VobSub subtitles.
    VobSub => "vobsub",
    /// Web Video Text Tracks subtitles.
    WebVtt => "webvtt",
}

crate::unit_tests!("subtitle_codec.test.rs");
