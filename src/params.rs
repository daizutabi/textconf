//! This module provides functionality for parsing and manipulating parameters in a string.
//! It includes the `ParameterReplacer` struct and the `ParameterIterator` struct to
//! iterate over parameters in a string.
//!
//! The `ParameterReplacer` struct is used to find and replace parameters within a string.
//! It collects all parameters and allows for their replacement with a given prefix.
//!
//! The `ParameterIterator` struct is used to iterate over the parameters in a string.
//! It can be created from a string slice and provides an iterator over the parameters.
//!
//! # Examples
//!
//! ```
//! use params::ParameterReplacer;
//!
//! let input = "{a}{b=2}{c:.2f=3.0}";
//! let replacer = ParameterReplacer::new(input);
//!
//! // Get all parameters
//! let parameters = replacer.parameters();
//! assert_eq!(parameters.len(), 3);
//! ```

use std::collections::HashSet;
use thiserror::Error;

/// The `Brace` struct represents a section of a string enclosed in curly braces `{}`.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Brace<'a> {
    /// The original string slice.
    input: &'a str,
    /// The starting position of the brace.
    start: usize,
    /// The ending position of the brace.
    end: usize,
}

impl<'a> Brace<'a> {
    /// Returns the content of the brace.
    pub fn content(&self) -> &'a str {
        &self.input[self.start + 1..self.end - 1]
    }
}

/// An iterator over the braces in a string.
#[derive(Debug, Clone, PartialEq)]
struct BraceIterator<'a> {
    /// The original string slice.
    input: &'a str,
    /// The current position in the string.
    start: usize,
}

impl<'a> BraceIterator<'a> {
    /// Creates a new BraceIterator instance.
    fn new(input: &'a str) -> Self {
        BraceIterator { input, start: 0 }
    }
}

impl<'a> Iterator for BraceIterator<'a> {
    type Item = Brace<'a>;

    /// Returns the next brace in the string.
    fn next(&mut self) -> Option<Self::Item> {
        let input = &self.input[self.start..];
        let mut start = None;
        let mut prev = None;

        for (index, c) in input.char_indices() {
            match c {
                '{' => {
                    if let Some(prev_char) = prev {
                        if prev_char == '{' {
                            prev = None;
                            start = None;
                            continue;
                        }
                    }
                    prev = Some(c);
                    start = Some(index);
                }
                '}' => {
                    if let Some(start_index) = start {
                        self.start += index + 1;
                        return Some(Brace {
                            input: self.input,
                            start: self.start - (index + 1 - start_index),
                            end: self.start,
                        });
                    }
                    prev = None;
                    start = None;
                }
                c => {
                    if c.is_whitespace() {
                        prev = None;
                        start = None;
                    } else if start.is_some() {
                        prev = Some(c)
                    }
                }
            }
        }
        None
    }
}

impl<'a> From<&'a str> for BraceIterator<'a> {
    fn from(input: &'a str) -> Self {
        BraceIterator::new(input)
    }
}

/// A struct representing a parameter in a string.
#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    /// The original string slice.
    input: &'a str,
    /// The starting position of the brace.
    start: usize,
    /// The ending position of the brace.
    end: usize,
    /// The name of the parameter.
    name: &'a str,
    /// The format of the parameter.
    format: Option<&'a str>,
    /// The default value of the parameter.
    default: Option<&'a str>,
}

impl<'a> Parameter<'a> {
    /// Returns the starting position of the brace.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending position of the brace.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the content of the brace.
    pub fn content(&self) -> &str {
        &self.input[self.start + 1..self.end - 1]
    }

    /// Returns the name of the parameter.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the format of the parameter.
    pub fn format(&self) -> Option<&str> {
        self.format
    }

    /// Returns the default value of the parameter.
    pub fn default(&self) -> Option<&str> {
        self.default
    }

    /// Returns the name of the parameter with the format.
    pub fn name_with_format(&self) -> &str {
        let end = self.end - self.default.map(|d| d.len() + 1).unwrap_or(0);
        &self.input[self.start + 1..end - 1]
    }
}

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Parameter name is empty: found {0}")]
    EmptyName(String),
}

impl<'a> TryFrom<Brace<'a>> for Parameter<'a> {
    type Error = ParameterError;

    fn try_from(brace: Brace<'a>) -> Result<Self, Self::Error> {
        let content = brace.content();

        let (name_default, default) = match content.split_once('=') {
            Some((name, default)) => (name, Some(default)),
            None => (content, None),
        };

        let (name, format) = match name_default.split_once(':') {
            Some((name, fmt)) => (name, Some(fmt)),
            None => (name_default, None),
        };

        if name.is_empty() {
            return Err(ParameterError::EmptyName(content.to_string()));
        }

        Ok(Parameter {
            input: brace.input,
            start: brace.start,
            end: brace.end,
            name,
            format,
            default,
        })
    }
}

/// An iterator over the parameters in a string.
pub struct ParameterIterator<'a> {
    /// The iterator over the braces in the string.
    brace_iter: BraceIterator<'a>,
}

impl<'a> ParameterIterator<'a> {
    /// Creates a new ParameterIterator instance.
    pub fn new(input: &'a str) -> Self {
        ParameterIterator {
            brace_iter: BraceIterator::new(input),
        }
    }
}

impl<'a> Iterator for ParameterIterator<'a> {
    type Item = Parameter<'a>;

    /// Returns the next parameter in the string.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(brace) = self.brace_iter.next() {
            if let Ok(param) = Parameter::try_from(brace) {
                return Some(param);
            }
        }
        None
    }
}

impl<'a> From<&'a str> for ParameterIterator<'a> {
    fn from(value: &'a str) -> Self {
        ParameterIterator::new(value)
    }
}

/// A struct that replaces parameters in a string with a specified prefix.
///
/// The `ParameterReplacer` struct is used to find and replace parameters within a string.
/// It collects all parameters and allows for their replacement with a given prefix.
///
/// # Examples
///
/// ```
/// use params::ParameterReplacer;
///
/// let input = "{a}{b=2}{c:.2f=3.0}";
/// let replacer = ParameterReplacer::new(input);
///
/// // Get all parameters
/// let parameters = replacer.parameters();
/// assert_eq!(parameters.len(), 3);
///
/// // Get parameters with default values
/// let parameters_with_default = replacer.parameters_with_default();
/// assert_eq!(parameters_with_default.len(), 2);
///
/// // Get names of parameters with default values
/// let names_with_default = replacer.names_with_default();
/// assert_eq!(names_with_default, vec!["b", "c"]);
///
/// // Replace parameters with a prefix, deleting default values
/// let replaced = replacer.replace("prefix.");
/// assert_eq!(replaced, "{a}{prefix.b}{prefix.c:.2f}");
/// ```
pub struct ParameterReplacer<'a> {
    input: &'a str,
    parameters: Vec<Parameter<'a>>,
}

impl<'a> ParameterReplacer<'a> {
    /// Creates a new ParameterReplacer instance.
    pub fn new(input: &'a str) -> Self {
        let parameters = ParameterIterator::new(input).collect();
        Self { input, parameters }
    }

    /// Returns all parameters.
    pub fn parameters(&self) -> &[Parameter<'a>] {
        &self.parameters
    }

    /// Returns all parameters with default values.
    pub fn parameters_with_default(&self) -> Vec<&Parameter<'a>> {
        self.parameters
            .iter()
            .filter(|&p| p.default().is_some())
            .collect()
    }

    /// Returns the names of parameters with default values.
    pub fn names_with_default(&self) -> Vec<&str> {
        self.parameters_with_default()
            .iter()
            .map(|&p| p.name())
            .collect()
    }

    /// Replaces parameters with a specified prefix, deleting default values.
    pub fn replace(&self, prefix: &str) -> String {
        let mut result = String::new();
        let mut last_index = 0;

        let names_with_default: HashSet<_> = self.names_with_default().into_iter().collect();

        for param in self.parameters.iter() {
            if names_with_default.contains(&param.name()) {
                result.push_str(&self.input[last_index..param.start()]);
                result.push('{');
                result.push_str(prefix);
                result.push_str(param.name_with_format());
                result.push('}');
                last_index = param.end();
            }
        }
        result.push_str(&self.input[last_index..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn brace_iter_simple() {
        let braces = BraceIterator::new("abc{def}ghi{jkl}mno");
        let vec: Vec<_> = braces.map(|brace| brace.content()).collect();
        assert_eq!(vec, vec!["def", "jkl"]);
    }

    #[rstest]
    #[case("{a{x}b{y}c}")]
    #[case("{{a{x}b{y}c}}")]
    #[case("a{{{x}}}b{{{y}}}c{{z}}d{{{{z}}}}")]
    #[case("{a{{{x}}}b{{{y}}}c{{z}}d}")]
    fn brace_iter_nested(#[case] input: &str) {
        let braces = BraceIterator::new(input);
        let vec: Vec<_> = braces.map(|brace| brace.content()).collect();
        assert_eq!(vec, vec!["x", "y"]);
    }
    #[test]
    fn test_parameter_creation() {
        let input = "{test}{test_default=default}{test_format:.2f}{test_both:.2f=3.14}";
        let params: Vec<Parameter> = ParameterIterator::from(input).collect();

        assert_eq!(params[0].name(), "test");
        assert_eq!(params[0].format(), None);
        assert_eq!(params[0].default(), None);

        assert_eq!(params[1].name(), "test_default");
        assert_eq!(params[1].format(), None);
        assert_eq!(params[1].default(), Some("default"));

        assert_eq!(params[2].name(), "test_format");
        assert_eq!(params[2].format(), Some(".2f"));
        assert_eq!(params[2].default(), None);

        assert_eq!(params[3].name(), "test_both");
        assert_eq!(params[3].format(), Some(".2f"));
        assert_eq!(params[3].default(), Some("3.14"));
    }

    #[test]
    fn test_parameter_content() {
        let input = "{test}{test_default=default}{test_format:.2f}{test_both:.2f=3.14}";
        let params: Vec<Parameter> = ParameterIterator::from(input).collect();

        assert_eq!(params[0].content(), "test");
        assert_eq!(params[1].content(), "test_default=default");
        assert_eq!(params[2].content(), "test_format:.2f");
        assert_eq!(params[3].content(), "test_both:.2f=3.14");
    }

    #[test]
    fn test_parameter_name_with_format() {
        let input = "{test}{test_default=default}{test_format:.2f}{test_both:.2f=3.14}";
        let params: Vec<Parameter> = ParameterIterator::from(input).collect();

        assert_eq!(params[0].name_with_format(), "test");
        assert_eq!(params[1].name_with_format(), "test_default");
        assert_eq!(params[2].name_with_format(), "test_format:.2f");
        assert_eq!(params[3].name_with_format(), "test_both:.2f");
    }

    #[test]
    fn test_replacer_parameters_with_default() {
        let input = "{a}{b=2}{c:.2f=3.0}";
        let replacer = ParameterReplacer::new(input);
        let parameters_with_default = replacer.parameters_with_default();
        assert_eq!(parameters_with_default.len(), 2);
        assert_eq!(parameters_with_default[0].name(), "b");
        assert_eq!(parameters_with_default[0].default(), Some("2"));
        assert_eq!(parameters_with_default[1].name(), "c");
        assert_eq!(parameters_with_default[1].default(), Some("3.0"));
    }

    #[test]
    fn test_replacer_parameters_with_default_complex() {
        let input = "{x}{y=42}{z:.3f=1.234}{w:.2%}{v=hello}{u:.1e=2.71828}";
        let replacer = ParameterReplacer::new(input);
        let parameters_with_default = replacer.parameters_with_default();
        assert_eq!(parameters_with_default.len(), 4);
        assert_eq!(parameters_with_default[0].name(), "y");
        assert_eq!(parameters_with_default[0].default(), Some("42"));
        assert_eq!(parameters_with_default[1].name(), "z");
        assert_eq!(parameters_with_default[1].default(), Some("1.234"));
        assert_eq!(parameters_with_default[2].name(), "v");
        assert_eq!(parameters_with_default[2].default(), Some("hello"));
        assert_eq!(parameters_with_default[3].name(), "u");
        assert_eq!(parameters_with_default[3].default(), Some("2.71828"));
    }

    #[test]
    fn test_replacer_names_with_default_complex() {
        let input = "{x}{y=42}{z:.3f=1.234}{w:.2%}{v=hello}{u:.1e=2.71828}";
        let replacer = ParameterReplacer::new(input);
        let names_with_default = replacer.names_with_default();
        assert_eq!(names_with_default.len(), 4);
        assert_eq!(names_with_default[0], "y");
        assert_eq!(names_with_default[1], "z");
        assert_eq!(names_with_default[2], "v");
        assert_eq!(names_with_default[3], "u");
    }

    #[rstest]
    #[case("{a}{b=2}{c:.2f=3.0}", "p.", "{a}{p.b}{p.c:.2f}")]
    #[case("{x}{y=42}{z:.3f=1.234}", "prefix_", "{x}{prefix_y}{prefix_z:.3f}")]
    #[case("{a}{b=2}{c:.2f=3.0}{d}", "pre_", "{a}{pre_b}{pre_c:.2f}{d}")]
    fn test_replacer(#[case] input: &str, #[case] prefix: &str, #[case] expected: &str) {
        let replacer = ParameterReplacer::new(input);
        let result = replacer.replace(prefix);
        assert_eq!(result, expected);
    }
}
