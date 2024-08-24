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
            c => {
                if c == ' ' || c == '\n' {
                    prev = None;
                    start = None;
                } else if start.is_some() {
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
        if !input.starts_with('{') || !input.ends_with('}') {
            return Err("Parameter must be enclosed in '{' and '}'");
        }

        let param = &input[1..input.len() - 1];
        let name_default: Vec<&str> = param.split('=').collect();
        if name_default.len() != 2 {
            return Err("Parameter format must contain exactly one '='");
        }

        let name_fmt: Vec<&str> = name_default[0].split(':').collect();
        if name_fmt[0].is_empty() {
            return Err("Parameter name is empty");
        }
        let name = name_fmt[0].to_string();

        let fmt = if name_fmt.len() == 2 && !name_fmt[1].is_empty() {
            Some(name_fmt[1].to_string())
        } else {
            None
        };

        let default = if !name_default[1].is_empty() {
            Some(name_default[1].to_string())
        } else {
            None
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

impl Parameter {
    /// Converts the parameter to a string representation, potentially losing some information.
    ///
    /// This method creates a string representation of the parameter, including the name and format (if present),
    /// but omits the default value. It's "lossy" because it doesn't include all the information stored in the Parameter.
    ///
    /// # Returns
    ///
    /// A `String` representing the parameter, either in the format "{name:format}" if a format is specified,
    /// or just the name if no format is present.
    fn to_string_lossy(&self) -> String {
        match &self.fmt {
            Some(fmt_value) => format!("{{{}:{}}}", self.name, fmt_value),
            None => format!("{{{}}}", self.name),
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

impl TryFrom<&str> for Parameters {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vec: Vec<_> = extract_braces(value)
            .into_iter()
            .filter_map(|s| Parameter::try_from(s).ok())
            .collect();

        let mut seen_names = std::collections::HashSet::with_capacity(vec.len());
        for param in &vec {
            if !seen_names.insert(&param.name) {
                return Err("Duplicate parameter name found");
            }
        }
        Ok(Parameters { vec })
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
    #[case("{=}")]
    #[case("{=1}")]
    #[case("{:=1}")]
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

    #[test]
    fn test_parameter_to_string_lossy() {
        let param = Parameter {
            name: "test".to_string(),
            fmt: Some(".2f".to_string()),
            default: Some("3.14".to_string()),
        };
        assert_eq!(param.to_string_lossy(), "{test:.2f}");
    }

    #[test]
    fn test_parameter_to_string_lossy_no_default() {
        let param = Parameter {
            name: "no_default".to_string(),
            fmt: Some(".3f".to_string()),
            default: None,
        };
        assert_eq!(param.to_string_lossy(), "{no_default:.3f}");
    }

    #[test]
    fn test_parameter_to_string_lossy_only_name() {
        let param = Parameter {
            name: "only_name".to_string(),
            fmt: None,
            default: None,
        };
        assert_eq!(param.to_string_lossy(), "{only_name}");
    }

    #[test]
    fn test_parameters_err() {
        assert!(Parameters::try_from("{a=1}{a=2}").is_err())
    }
}
