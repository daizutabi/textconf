//! This module provides functionality for splitting and accumulating parts of a string
//! based on a specified delimiter. It includes the `SplitAccumulate` struct and the
//! `SplitAccumulateExt` trait to extend the functionality of the `str` type.
//!
//! The `SplitAccumulate` struct allows for the creation of an iterator that returns
//! accumulated substrings each time it is called. This can be useful for scenarios
//! where you need to progressively build up a string from its parts.
//!
//! The `SplitAccumulateExt` trait adds the `split_accumulate` method to the `str` type,
//! enabling easy creation of `SplitAccumulate` instances directly from string slices.
//!
//! # Examples
//!
//! ```
//! use path::SplitAccumulateExt;
//!
//! let input = "a,b,c";
//! let mut iter = input.split_accumulate(',');
//!
//! assert_eq!(iter.next(), Some("a".to_string()));
//! assert_eq!(iter.next(), Some("a,b".to_string()));
//! assert_eq!(iter.next(), Some("a,b,c".to_string()));
//! assert_eq!(iter.next(), None);
//! ```

/// A struct that accumulates parts of a string split by a delimiter.
struct SplitAccumulate<'a> {
    /// The character used to split the string.
    delimiter: char,
    /// An iterator over the parts of the string split by the delimiter.
    parts: std::str::Split<'a, char>,
    /// The current accumulated string.
    current: String,
}

impl<'a> SplitAccumulate<'a> {
    /// Creates a new AccumulateSplit instance.
    fn new(input: &'a str, delimiter: char) -> Self {
        SplitAccumulate {
            delimiter,
            parts: input.split(delimiter),
            current: String::new(),
        }
    }
}

impl<'a> Iterator for SplitAccumulate<'a> {
    type Item = String;

    /// Returns the next accumulated substring.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(part) = self.parts.next() {
            if !self.current.is_empty() {
                self.current.push(self.delimiter);
            }
            self.current.push_str(part);
            Some(self.current.clone())
        } else {
            None
        }
    }
}

/// SplitAccumulateExt trait adds the split_accumulate method to the str type.
trait SplitAccumulateExt {
    fn split_accumulate(&self, delimiter: char) -> SplitAccumulate;
}

impl SplitAccumulateExt for str {
    /// Splits and accumulates the string, returning an AccumulateSplit.
    fn split_accumulate(&self, delimiter: char) -> SplitAccumulate {
        SplitAccumulate::new(self, delimiter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("user.profile.age", '.', vec!["user", "user.profile", "user.profile.age"])]
    #[case("a.b.c.d", '.', vec!["a", "a.b", "a.b.c", "a.b.c.d"])]
    #[case("", '.', vec![""])]
    #[case(".", '.', vec!["", ""])]
    fn test_split_accumulate(
        #[case] input: &str,
        #[case] delimiter: char,
        #[case] expected: Vec<&str>,
    ) {
        let splitter = input.split_accumulate(delimiter);
        let result: Vec<String> = splitter.collect();
        assert_eq!(result, expected);
    }
}
