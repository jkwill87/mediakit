//! Defines movie and television media categories.

crate::macros::convertable_enum!(
    /// Media file types recognized by the library.
    MediaType,
    /// A feature film or standalone video.
    Movie => "movie",
    /// A television series episode.
    Television => "television",
    /// An unclassified media file.
    Unknown => "unknown",
);
