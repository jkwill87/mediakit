//! Defines normalized media release-source values.

crate::macros::convertable_enum! {
    /// Release sources recognized by the library.
    ReleaseSource,
    /// Blu-ray disc.
    BluRay => "bluray",
    /// DVD disc.
    Dvd => "dvd",
    /// HDTV broadcast capture.
    HDtv => "hdtv",
    /// Telecine film transfer.
    Telecine => "telecine",
    /// Web stream rip.
    WebRip => "webrip",
    /// Web download.
    WebDl => "webdl",
}
