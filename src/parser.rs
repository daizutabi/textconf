//! This module provides functionality for parsing and manipulating text-base
//! data structures.
//!
//! It includes functions for extracting substrings enclosed in balanced braces,
//! as well as structures for representing variables and their default values.
//! These tools are particularly useful for processing text-based data structures
//! that use brace notation.

/// Extracts all substrings enclosed in balanced braces from the input string.
///
/// This function scans the input string and identifies all substrings that are
/// enclosed within balanced curly braces {}. It handles nested braces correctly.
///
/// # Arguments
///
/// * `input` - A string slice that may contain substrings enclosed in braces.
///
/// # Returns
///
/// A vector of string slices, where each slice is a substring from the input
/// that was enclosed in balanced braces, including the braces themselves.
pub fn extract_braces(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = None;
    let mut depth = 0;

    for (index, c) in input.char_indices() {
        match c {
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(start_index) = start {
                            result.push(&input[start_index..=index]);
                        }
                        start = None;
                    }
                }
            }
            _ => {}
        }
    }

    result
}

/// Represents a variable with an optional default value.
///
/// This struct is used to store information about a variable, including its name
/// and an optional default value.
///
/// # Fields
///
/// * `name` - The name of the variable.
/// * `default` - An optional default value for the variable.
#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    default: Option<String>,
}

impl TryFrom<&str> for Variable {
    type Error = &'static str;

    /// Attempts to create a `Variable` from a string slice.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that should represent a variable.
    ///
    /// # Returns
    ///
    /// * `Ok(Variable)` if the input is valid and can be parsed into a `Variable`.
    /// * `Err(&'static str)` if the input is invalid, with an error message explaining why.
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let left_count = input.chars().take_while(|&c| c == '{').count();
        let right_count = input.chars().rev().take_while(|&c| c == '}').count();

        if left_count != right_count || left_count % 2 == 0 {
            return Err("Invalid input format: mismatched or even number of braces");
        }

        let variable = &input[left_count..input.len() - right_count];
        if variable.contains('{') || variable.contains('}') || variable.contains(' ') {
            return Err("Invalid characters in variable name: contains braces or spaces");
        }

        let parts: Vec<&str> = variable.split('=').collect();
        if parts.len() != 2 || parts[0].is_empty() {
            return Err("Variable format must contain '=' and a non-empty name");
        }

        let name = parts[0].split(':').next().unwrap().to_string();
        if name.is_empty() {
            return Err("Variable name is empty");
        }
        let default = match parts[1] {
            "" => None,
            x => Some(x.to_string()),
        };
        Ok(Variable { name, default })
    }
}

/// Represents a collection of variables.
///
/// This struct holds a vector of `Variable` instances and provides
/// convenient access to them through the `Deref` trait.
#[derive(Debug, PartialEq)]
pub struct Variables {
    vec: Vec<Variable>,
}

impl std::ops::Deref for Variables {
    type Target = Vec<Variable>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl From<&str> for Variables {
    /// Converts a string slice into a `Variables` instance.
    ///
    /// This function extracts all valid variable definitions from the input string
    /// and creates a `Variables` instance containing them.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that may contain variable definitions enclosed in braces.
    ///
    /// # Returns
    ///
    /// A `Variables` instance containing all valid variables extracted from the input string.
    fn from(value: &str) -> Self {
        let vec: Vec<_> = extract_braces(value)
            .into_iter()
            .filter_map(|s| Variable::try_from(s).ok())
            .collect();
        Variables { vec }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn extract_braces_simple() {
        assert_eq!(
            extract_braces("abc{def}ghi{jkl}mno"),
            vec!["{def}", "{jkl}"]
        );
    }

    #[test]
    fn extract_braces_nested() {
        assert_eq!(extract_braces("a{b{c}d{e}}f"), vec!["{b{c}d{e}}"]);
    }

    #[rstest]
    #[case("abc")]
    #[case("a{b{c}d")]
    #[case("a{b{c}d{e}f")]
    fn extract_braces_empty(#[case] input: &str) {
        assert_eq!(extract_braces(input), Vec::<&str>::new());
    }

    #[test]
    fn test_extract_braces_unbalanced_close() {
        assert_eq!(extract_braces("a}b{c}d}"), vec!["{c}"]);
    }

    #[test]
    fn extract_braces_unbalanced_mixed() {
        assert_eq!(extract_braces("{a}{b}}c{{d}e"), vec!["{a}", "{b}"]);
    }

    #[test]
    fn extract_braces_sequential_nested() {
        let result = extract_braces("a{{c}}{d{e}}{f{g{h}}}i{{{j}}}");
        assert_eq!(result, vec!["{{c}}", "{d{e}}", "{f{g{h}}}", "{{{j}}}"]);
    }

    #[rstest]
    #[case("abc")]
    #[case("{a}")]
    #[case("{{a}}")]
    #[case("{{a=3}}")]
    #[case("{{a}")]
    #[case("{{{a{b}}}}")]
    #[case("{a{b}}")]
    #[case("{a b}")]
    #[case("{=}")]
    #[case("{:=1}")]
    #[case("{=1}")]
    fn variable_err(#[case] value: &str) {
        assert!(Variable::try_from(value).is_err());
    }

    #[rstest]
    #[case("{a=}")]
    #[case("{a:.2f=}")]
    fn variable_without_default(#[case] value: &str) {
        let result = Variable::try_from(value).unwrap();
        assert_eq!(result.name, "a".to_string());
        assert_eq!(result.default, None)
    }

    #[rstest]
    #[case("{a=1.0}")]
    #[case("{a:.2f=1.0}")]
    fn variable_with_default(#[case] value: &str) {
        let result = Variable::try_from(value).unwrap();
        assert_eq!(result.name, "a".to_string());
        assert_eq!(result.default, Some("1.0".to_string()));
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("Hello, {name}")]
    fn variables_empty(#[case] value: &str) {
        assert!(Variables::from(value).is_empty());
    }

    #[test]
    fn variables_with_defaults() {
        let variables = Variables::from("{name=John} is {age=30} years old.");
        let mut it = variables.iter();
        assert_eq!(
            it.next(),
            Some(&Variable {
                name: "name".to_string(),
                default: Some("John".to_string())
            })
        );
        assert_eq!(
            it.next(),
            Some(&Variable {
                name: "age".to_string(),
                default: Some("30".to_string())
            })
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn variables_mixed() {
        let variables =
            Variables::from("Hello, {name=}! Today is {day} and it's {temperature=20} degrees.");
        let mut it = variables.iter();
        assert_eq!(
            it.next(),
            Some(&Variable {
                name: "name".to_string(),
                default: None,
            })
        );
        assert_eq!(
            it.next(),
            Some(&Variable {
                name: "temperature".to_string(),
                default: Some("20".to_string())
            })
        );
        assert_eq!(it.next(), None);
    }
}
