use std::collections::HashMap;

/// AccumulateSplit struct is an iterator that splits and accumulates a string.
struct SplitAccumulate<'a> {
    sep: char,
    parts: std::str::Split<'a, char>,
    current: String,
}

impl<'a> SplitAccumulate<'a> {
    /// Creates a new AccumulateSplit instance.
    fn new(input: &'a str, sep: char) -> Self {
        SplitAccumulate {
            sep,
            parts: input.split(sep),
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
                self.current.push(self.sep);
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
    fn split_accumulate(&self, sep: char) -> SplitAccumulate;
}

impl SplitAccumulateExt for str {
    /// Splits and accumulates the string, returning an AccumulateSplit.
    fn split_accumulate(&self, sep: char) -> SplitAccumulate {
        SplitAccumulate::new(self, sep)
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
    fn test_split_accumulate(#[case] input: &str, #[case] sep: char, #[case] expected: Vec<&str>) {
        let splitter = input.split_accumulate(sep);
        let result: Vec<String> = splitter.collect();
        assert_eq!(result, expected);
    }
}
