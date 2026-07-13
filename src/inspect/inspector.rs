//! Defines the common contract for media metadata inspection.

use crate::meta::Tag;

/// A trait for inspecting media files and extracting metadata [`Tag`]s.
pub trait Inspector {
    /// Runs the inspection pipeline and returns the updated inspector.
    fn analyze(self) -> Self;
    /// Returns references to all extracted [`Tag`]s.
    fn tags(&self) -> Vec<&Tag>;
}
