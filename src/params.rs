use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Brace<'a> {
    input: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Brace<'a> {
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
pub struct Parameter<'a> {
    input: &'a str,
    start: usize,
    end: usize,
    name: &'a str,
    fmt: Option<&'a str>,
    default: Option<&'a str>,
}

impl<'a> Parameter<'a> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn content(&self) -> &str {
        &self.input[self.start + 1..self.end - 1]
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn fmt(&self) -> Option<&str> {
        self.fmt
    }

    pub fn default(&self) -> Option<&str> {
        self.default
    }

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

        let (name, fmt) = match name_default.split_once(':') {
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
            fmt,
            default,
        })
    }
}

pub struct ParameterIterator<'a> {
    brace_iter: BraceIterator<'a>,
}

impl<'a> ParameterIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        ParameterIterator {
            brace_iter: BraceIterator::new(input),
        }
    }
}

impl<'a> Iterator for ParameterIterator<'a> {
    type Item = Parameter<'a>;

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

pub struct ParameterReplacer<'a> {
    input: &'a str,
    parameters: Vec<Parameter<'a>>,
}

impl<'a> ParameterReplacer<'a> {
    pub fn new(input: &'a str) -> Self {
        let parameters = ParameterIterator::new(input).collect();
        Self { input, parameters }
    }

    pub fn parameters(&self) -> &[Parameter<'a>] {
        &self.parameters
    }

    pub fn parameters_with_default(&self) -> Vec<&Parameter<'a>> {
        self.parameters
            .iter()
            .filter(|&p| p.default().is_some())
            .collect()
    }

    pub fn names_with_default(&self) -> Vec<&str> {
        self.parameters_with_default()
            .iter()
            .map(|&p| p.name())
            .collect()
    }

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
        assert_eq!(params[0].default(), None);
        assert_eq!(params[0].fmt(), None);

        assert_eq!(params[1].name(), "test_default");
        assert_eq!(params[1].default(), Some("default"));
        assert_eq!(params[1].fmt(), None);

        assert_eq!(params[2].name(), "test_format");
        assert_eq!(params[2].default(), None);
        assert_eq!(params[2].fmt(), Some(".2f"));

        assert_eq!(params[3].name(), "test_both");
        assert_eq!(params[3].default(), Some("3.14"));
        assert_eq!(params[3].fmt(), Some(".2f"));
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
