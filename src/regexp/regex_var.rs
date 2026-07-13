//! Wraps Pike VM compilation, captures, and reusable match state.

use super::RegexMatch;
use regex_automata::PatternID;
use regex_automata::nfa::thompson::pikevm::{Cache, PikeVM};
use regex_automata::util::captures::Captures;

/// RegexVar is a wrapper around the PikeVM regex engine.
///
/// It provides a more ergonomic interface for working with regular expressions as well as a smaller
/// web assembly binary size than regex::Regex.
pub struct RegexVar {
    needle: &'static str,
    vm: PikeVM,
    cache: Cache,
    captures: Captures,
}

impl RegexVar {
    pub fn new(needle: &'static str) -> Self {
        let vm = PikeVM::new(needle).unwrap();
        let cache = vm.create_cache();
        let captures = vm.create_captures();
        Self {
            needle,
            vm,
            cache,
            captures,
        }
    }

    pub fn labels(&self) -> Vec<&'static str> {
        self.captures
            .group_info()
            .pattern_names(PatternID::ZERO)
            .flatten()
            .map(|name| {
                let start = self.needle.find(name).unwrap();
                let end = start + name.len();
                &self.needle[start..end]
            })
            .collect()
    }

    pub fn get(&self, label: &'static str) -> Option<RegexMatch> {
        self.captures
            .get_group_by_name(label)
            .map(|span| RegexMatch {
                label: Some(label),
                start: span.start,
                end: span.end,
            })
    }

    pub fn captures(&self) -> Vec<RegexMatch> {
        self.captures
            .iter()
            .flatten()
            .map(|span| RegexMatch {
                label: None,
                start: span.start,
                end: span.end,
            })
            .collect()
    }

    pub fn labeled_captures(&self) -> Vec<RegexMatch> {
        self.labels()
            .into_iter()
            .filter_map(|label| self.get(label))
            .collect()
    }

    pub fn search(&mut self, haystack: &str) -> bool {
        self.vm
            .captures(&mut self.cache, haystack, &mut self.captures);
        self.captures.is_match()
    }
}
