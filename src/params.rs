#[derive(Debug, Copy, Clone, PartialEq)]
struct Brace<'a> {
    input: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Brace<'a> {
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }

    pub fn as_str(&self) -> &'a str {
        &self.input[self.range()]
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Braces<'a> {
    vec: Vec<Brace<'a>>,
}

impl<'a> Braces<'a> {
    fn new(input: &'a str) -> Self {
        let mut vec = Vec::new();
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
                        let brace = Brace {
                            input,
                            start: start_index,
                            end: index + 1,
                        };
                        vec.push(brace);
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
        Braces { vec }
    }
}

impl<'a> From<&'a str> for Braces<'a> {
    fn from(input: &'a str) -> Self {
        Braces::new(input)
    }
}

impl<'a> std::ops::Deref for Braces<'a> {
    type Target = Vec<Brace<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    name: String,
    fmt: Option<String>,
    default: String,
}

impl Parameter {
    pub fn name(&self) -> &str {
        &self.name
    }
    // pub fn fmt(&self) -> Option<&str> {
    //     self.fmt.as_deref()
    // }
    pub fn default(&self) -> &str {
        self.default.as_str()
    }
}

impl TryFrom<&str> for Parameter {
    // type Error = &'static str;
    type Error = anyhow::Error;

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
            anyhow::bail!(
                "Parameter must be enclosed in '{{' and '}}': found {}",
                input
            );
        }

        let param = &input[1..input.len() - 1];
        let name_default: Vec<&str> = param.split('=').collect();
        if name_default.len() != 2 {
            anyhow::bail!(
                "Parameter format must contain exactly one '=': found {}",
                input
            );
        }

        if name_default[1].is_empty() {
            anyhow::bail!("Parameter default is empty: found {}", input);
        }
        let default = name_default[1].to_string();

        let name_fmt: Vec<&str> = name_default[0].split(':').collect();
        if name_fmt[0].is_empty() {
            anyhow::bail!("Parameter name is empty: found {}", input);
        }
        let name = name_fmt[0].to_string();

        let fmt = if name_fmt.len() == 2 && !name_fmt[1].is_empty() {
            Some(name_fmt[1].to_string())
        } else {
            None
        };

        Ok(Parameter { name, fmt, default })
    }
}

impl<'a> TryFrom<&'a Brace<'a>> for Parameter {
    type Error = anyhow::Error;

    fn try_from(brace: &'a Brace<'a>) -> Result<Self, Self::Error> {
        Parameter::try_from(brace.as_str())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_fmt = match &self.fmt {
            Some(fmt_value) => format!("{}:{}", self.name, fmt_value),
            None => format!("{}", self.name),
        };
        write!(f, "{{{}={}}}", name_fmt, self.default)
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
    type Error = anyhow::Error;

    /// Attempts to create a `Parameters` instance from a string slice.
    ///
    /// This function parses the input string, extracting valid parameters enclosed in braces.
    /// It then checks for duplicate parameter names, which are not allowed.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice containing the parameter definitions.
    ///
    /// # Returns
    ///
    /// * `Ok(Parameters)` if parsing is successful and there are no duplicate names.
    /// * `Err` if there's a parsing error or if duplicate parameter names are found.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::Parameters;
    ///
    /// let params = Parameters::try_from("{a=1}{b:.2f=3.14}").unwrap();
    /// assert_eq!(params.len(), 2);
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let braces: Braces = value.into();
        let vec: Vec<_> = braces
            .iter()
            .filter_map(|s| Parameter::try_from(s).ok())
            .collect();

        let mut seen_names = std::collections::HashSet::with_capacity(vec.len());
        for param in &vec {
            if !seen_names.insert(&param.name) {
                anyhow::bail!("Duplicate parameter name found");
            }
        }
        Ok(Parameters { vec })
    }
}

impl Parameters {
    pub fn replace_lossy(&self, input: &str) -> String {
        let mut result = input.to_string();
        for param in &self.vec {
            let from = param.to_string();
            let to = param.to_string_lossy();
            result = result.replace(&from, &to);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn braces_simple() {
        let braces = Braces::new("abc{def}ghi{jkl}mno");
        let vec: Vec<_> = braces.iter().map(Brace::as_str).collect();
        assert_eq!(vec, vec!["{def}", "{jkl}"]);
    }

    #[rstest]
    #[case("{a{x}b{y}c}")]
    #[case("{{a{x}b{y}c}}")]
    #[case("a{{{x}}}b{{{y}}}c{{z}}d{{{{z}}}}")]
    #[case("{a{{{x}}}b{{{y}}}c{{z}}d}")]
    fn extract_braces_nested(#[case] input: &str) {
        let braces = Braces::new(input);
        let vec: Vec<_> = braces.iter().map(Brace::as_str).collect();
        assert_eq!(vec, vec!["{x}", "{y}"]);
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
    #[case("{a=1}", "a", None, "1")]
    #[case("{b:.2f=3.14}", "b", Some(".2f"), "3.14")]
    fn parameter_ok(
        #[case] input: &str,
        #[case] name: &str,
        #[case] fmt: Option<&str>,
        #[case] default: &str,
    ) {
        let param = Parameter::try_from(input).unwrap();
        assert_eq!(param.name, name);
        assert_eq!(param.fmt.as_deref(), fmt);
        assert_eq!(param.default, default);
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
            default: "3.14".to_string(),
        };
        assert_eq!(param.to_string_lossy(), "{test:.2f}");
    }

    #[test]
    fn test_parameter_to_string_lossy_no_fmt() {
        let param = Parameter {
            name: "name".to_string(),
            fmt: None,
            default: "a".to_string(),
        };
        assert_eq!(param.to_string_lossy(), "{name}");
    }

    #[test]
    fn test_parameters_err() {
        assert!(Parameters::try_from("{a=1}{a=2}").is_err())
    }

    #[test]
    fn test_replace() {
        let input = "{a}{b=2}{c:.2f=3.0}";
        let params = Parameters::try_from(input).unwrap();
        assert_eq!(params.replace_lossy(input), "{a}{b}{c:.2f}")
    }
}
