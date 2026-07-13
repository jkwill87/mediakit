//! Maps recognized media formats to MIME types.

pub fn parse_mime_type(s: &str) -> Option<&'static str> {
    crate::meta::fields::MediaFormat::from_extension(s).map(|format| format.mime_type())
}
