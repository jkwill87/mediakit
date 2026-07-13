//! Represents labeled spans produced by regular-expression matching.

/// RegexMatch is a struct that represents the span of a match in a haystack.
pub struct RegexMatch {
    pub label: Option<&'static str>,
    pub start: usize,
    pub end: usize,
}

impl RegexMatch {
    pub const fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    pub fn template<'h>(&self, haystack: &'h str) -> &'h str {
        &haystack[self.range()]
    }
}
