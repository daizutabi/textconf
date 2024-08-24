/// Extracts substrings enclosed in single braces from the input string.
///
/// This function scans the input string and identifies substrings that are
/// enclosed in single braces ('{' and '}'). It handles nested braces by
/// ignoring double braces ('{{' and '}}').
///
/// # Arguments
///
/// * `input` - A string slice that may contain substrings enclosed in braces.
///
/// # Returns
///
/// A vector of string slices, where each slice is a substring from the input
/// that was enclosed in single braces, including the braces themselves.
///
/// # Examples
///
/// ```
/// let input = "Hello, {name}! Your score is {{score}}.";
/// let result = extract_braces(input);
/// assert_eq!(result, vec!["{name}"]);
/// ```
pub fn extract_braces(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
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
                    result.push(&input[start_index..=index]);
                }
                prev = None;
                start = None;
            }
            _ => {
                if start.is_some() {
                    prev = Some(c)
                }
            }
        }
    }

    result
}

/// Represents a parameter with a name, optional format, and optional default value.
///
/// # Fields
///
/// * `name` - The name of the parameter.
/// * `fmt` - An optional format specifier for the parameter.
/// * `default` - An optional default value for the parameter.
#[derive(Debug, PartialEq)]
pub struct Parameter {
    name: String,
    fmt: Option<String>,
    default: Option<String>,
}

impl TryFrom<&str> for Parameter {
    type Error = &'static str;

    /// Attempts to create a `Parameter` from a string slice.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that should represent a parameter.
    ///
    /// # Returns
    ///
    /// * `Ok(Parameter)` if the input is valid and can be parsed into a `Parameter`.
    /// * `Err(&'static str)` if the input is invalid, with an error message explaining why.
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let left_count = input.chars().take_while(|&c| c == '{').count();
        let right_count = input.chars().rev().take_while(|&c| c == '}').count();

        if left_count != right_count || left_count % 2 == 0 {
            return Err("Invalid input format: mismatched or even number of braces");
        }

        let parameter = &input[left_count..input.len() - right_count];
        if parameter.contains('{') || parameter.contains('}') || parameter.contains(' ') {
            return Err("Invalid characters in parameter name: contains braces or spaces");
        }

        let name_default: Vec<&str> = parameter.split('=').collect();
        if name_default.len() != 2 || name_default[0].is_empty() {
            return Err("Parameter format must contain '=' and a non-empty name");
        }

        let name_fmt: Vec<&str> = name_default[0].split(':').collect();
        if name_fmt[0].is_empty() {
            return Err("Parameter name is empty");
        }
        let name = name_fmt[0].to_string();

        let fmt = if name_fmt.len() == 1 || name_fmt[1].is_empty() {
            None
        } else {
            Some(name_fmt[1].to_string())
        };

        let default = if name_default[1].is_empty() {
            None
        } else {
            Some(name_default[1].to_string())
        };

        Ok(Parameter { name, fmt, default })
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_fmt = match &self.fmt {
            Some(fmt_value) => format!("{}:{}", self.name, fmt_value),
            None => format!("{}", self.name),
        };

        match &self.default {
            Some(default_value) => write!(f, "{{{}={}}}", name_fmt, default_value),
            None => write!(f, "{{{}=}}", name_fmt),
        }
    }
}

/// Represents a collection of parameters.
///
/// This struct holds a vector of `Parameter` instances and provides
/// convenient access to them through the `Deref` trait.
#[derive(Debug, PartialEq)]
pub struct Parameters {
    vec: Vec<Parameter>,
}

impl std::ops::Deref for Parameters {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl From<&str> for Parameters {
    /// Converts a string slice into a `Parameters` instance.
    ///
    /// This function extracts all valid parameter definitions from the input string
    /// and creates a `Parameters` instance containing them.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that may contain parameter definitions enclosed in braces.
    ///
    /// # Returns
    ///
    /// A `Parameters` instance containing all valid parameters extracted from the input string.
    fn from(value: &str) -> Self {
        let vec: Vec<_> = extract_braces(value)
            .into_iter()
            .filter_map(|s| Parameter::try_from(s).ok())
            .collect();
        Parameters { vec }
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

    #[rstest]
    #[case("{a{x}b{y}c}")]
    #[case("{{a{x}b{y}c}}")]
    #[case("a{{{x}}}b{{{y}}}c{{z}}d{{{{z}}}}")]
    #[case("{a{{{x}}}b{{{y}}}c{{z}}d}")]
    fn extract_braces_nested(#[case] input: &str) {
        assert_eq!(extract_braces(input), vec!["{x}", "{y}"]);
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
    fn parameter_err(#[case] value: &str) {
        assert!(Parameter::try_from(value).is_err());
    }

    #[rstest]
    #[case("{a=1}", "a", None, Some("1"))]
    #[case("{b:.2f=3.14}", "b", Some(".2f"), Some("3.14"))]
    fn parameter_ok(
        #[case] input: &str,
        #[case] name: &str,
        #[case] fmt: Option<&str>,
        #[case] default: Option<&str>,
    ) {
        let param = Parameter::try_from(input).unwrap();
        assert_eq!(param.name, name);
        assert_eq!(param.fmt.as_deref(), fmt);
        assert_eq!(param.default.as_deref(), default);
    }

    #[rstest]
    #[case("{a=1}")]
    #[case("{a:.2f=1.0}")]
    fn parameter_display(#[case] input: &str) {
        let param = Parameter::try_from(input).unwrap();
        assert_eq!(format!("{}", param), input);
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("Hello, {name}")]
    fn parameters_empty(#[case] value: &str) {
        assert!(Parameters::from(value).is_empty());
    }
}
