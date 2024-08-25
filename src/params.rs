use thiserror::Error;

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

    pub fn content(&self) -> &'a str {
        &self.input[self.start + 1..self.end - 1]
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BraceIterator<'a> {
    input: &'a str,
    start: usize,
}

impl<'a> BraceIterator<'a> {
    fn new(input: &'a str) -> Self {
        BraceIterator { input, start: 0 }
    }
}

impl<'a> Iterator for BraceIterator<'a> {
    type Item = Brace<'a>;

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

#[derive(Debug, PartialEq)]
pub struct Parameter {
    name: String,
    fmt: Option<String>,
    default: Option<String>,
}

impl Parameter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub fn to_string_without_default(&self, prefix: Option<&str>) -> String {
        let prefix = prefix.unwrap_or("");
        let name = format!("{}{}", prefix, self.name);
        match &self.fmt {
            Some(fmt_value) => format!("{{{}:{}}}", name, fmt_value),
            None => format!("{{{}}}", name),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Parameter name is empty: found {0}")]
    EmptyName(String),
}

impl<'a> TryFrom<&'a Brace<'a>> for Parameter {
    type Error = ParameterError;

    fn try_from(brace: &'a Brace<'a>) -> Result<Self, Self::Error> {
        let content = brace.content();

        let (name_default, default) = match content.split_once('=') {
            Some((name, default)) => (name, Some(default.to_string())),
            None => (content, None),
        };

        let (name, fmt) = match name_default.split_once(':') {
            Some((name, fmt)) => (name, Some(fmt.to_string())),
            None => (name_default, None),
        };

        if name.is_empty() {
            return Err(ParameterError::EmptyName(content.to_string()));
        }

        Ok(Parameter {
            name: name.to_string(),
            fmt,
            default,
        })
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_fmt = match &self.fmt {
            Some(fmt_value) => format!("{}:{}", self.name, fmt_value),
            None => self.name.to_string(),
        };

        match &self.default {
            Some(default_value) => write!(f, "{{{}={}}}", name_fmt, default_value),
            None => write!(f, "{{{}}}", name_fmt),
        }
    }
}

pub struct Parameters<'a> {
    braces: BraceIterator<'a>,
}

impl<'a> Parameters<'a> {
    pub fn new(input: &'a str) -> Self {
        Parameters {
            braces: BraceIterator::new(input),
        }
    }

    pub fn replace_without_default(&self, input: &str, prefix: Option<&str>) -> String {
        let mut braces = BraceIterator::new(input);
        let mut parameters = Vec::new();

        while let Some(brace) = braces.next() {
            if let Ok(param) = Parameter::try_from(&brace) {
                parameters.push(param);
            }
        }

        let names_with_default: std::collections::HashSet<_> = parameters
            .iter()
            .filter(|p| p.default().is_some())
            .map(|p| p.name().to_string())
            .collect();

        let mut result = String::new();
        let mut last_index = 0;

        for param in parameters {
            if names_with_default.contains(param.name()) {
                let from = param.to_string();
                let to = param.to_string_without_default(prefix);
                if let Some(start) = input[last_index..].find(&from) {
                    let start = last_index + start;
                    result.push_str(&input[last_index..start]);
                    result.push_str(&to);
                    last_index = start + from.len();
                }
            }
        }
        result.push_str(&input[last_index..]);
        result
    }
    // pub fn replace_without_default(&self, input: &str, prefix: Option<&str>) -> String {
    //     let names_with_default: std::collections::HashSet<_> = self
    //         .into_iter()
    //         .filter(|p| p.default().is_some())
    //         .map(|p| p.name().to_string())
    //         .collect();

    //     let mut result = String::new();
    //     let mut last_index = 0;

    //     for param in self.clone() {
    //         if names_with_default.contains(param.name()) {
    //             let from = param.to_string();
    //             let to = param.to_string_without_default(prefix);
    //             if let Some(start) = input[last_index..].find(&from) {
    //                 let start = last_index + start;
    //                 result.push_str(&input[last_index..start]);
    //                 result.push_str(&to);
    //                 last_index = start + from.len();
    //             }
    //         }
    //     }
    //     result.push_str(&input[last_index..]);
    //     result
    // }
}

impl<'a> Iterator for Parameters<'a> {
    type Item = Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(brace) = self.braces.next() {
            if let Ok(param) = Parameter::try_from(&brace) {
                return Some(param);
            }
        }
        None
    }
}

impl<'a> From<&'a str> for Parameters<'a> {
    fn from(value: &'a str) -> Self {
        Parameters::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn braces_simple() {
        let braces = BraceIterator::new("abc{def}ghi{jkl}mno");
        let vec: Vec<_> = braces.map(|brace| brace.as_str()).collect();
        assert_eq!(vec, vec!["{def}", "{jkl}"]);
    }

    #[rstest]
    #[case("{a{x}b{y}c}")]
    #[case("{{a{x}b{y}c}}")]
    #[case("a{{{x}}}b{{{y}}}c{{z}}d{{{{z}}}}")]
    #[case("{a{{{x}}}b{{{y}}}c{{z}}d}")]
    fn extract_braces_nested(#[case] input: &str) {
        let braces = BraceIterator::new(input);
        let vec: Vec<_> = braces.map(|brace| brace.as_str()).collect();
        assert_eq!(vec, vec!["{x}", "{y}"]);
    }

    #[test]
    fn test_parameter_to_string_without_default() {
        let param = Parameter {
            name: "test".to_string(),
            fmt: Some(".2f".to_string()),
            default: Some("3.14".to_string()),
        };
        assert_eq!(param.to_string_without_default(None), "{test:.2f}");
        assert_eq!(param.to_string_without_default(Some("p.")), "{p.test:.2f}");
    }

    #[test]
    fn test_parameter_to_string_without_default_no_fmt() {
        let param = Parameter {
            name: "name".to_string(),
            fmt: None,
            default: Some("a".to_string()),
        };
        assert_eq!(param.to_string_without_default(None), "{name}");
        assert_eq!(param.to_string_without_default(Some("p.")), "{p.name}");
    }

    #[test]
    fn test_replace_no_prefix() {
        let input = "{a}{b=2}{c:.2f=3.0}";
        let params = Parameters::from(input);
        assert_eq!(params.replace_without_default(input, None), "{a}{b}{c:.2f}")
    }

    #[test]
    fn test_replace_with_prefix() {
        let input = "{a}{b=2}{c:.2f=3.0}{a:.2f}{b:.1f}{c}{{b}}";
        let params = Parameters::from(input);
        assert_eq!(
            params.replace_without_default(input, Some("p.")),
            "{a}{p.b}{p.c:.2f}{a:.2f}{p.b:.1f}{p.c}{{b}}"
        )
    }
}
