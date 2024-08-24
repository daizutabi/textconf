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
/// * `name` - A String containing the name of the variable.
/// * `default` - An Option<String> that holds the default value of the variable, if any.
#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    default: Option<String>,
}

/// Extracts a variable from a string input.
///
/// This function parses a string input to extract a variable, which may include a default value.
/// The input string should be in the format "{variable_name=default_value}" or "{variable_name=}".
///
/// # Arguments
///
/// * `input` - A string slice that holds the input to be parsed.
///
/// # Returns
///
/// * `Some(Variable)` if a valid variable is found in the input.
/// * `None` if the input does not contain a valid variable format.
pub fn get_variable(input: &str) -> Option<Variable> {
    let left_count = input.chars().take_while(|&c| c == '{').count();
    let right_count = input.chars().rev().take_while(|&c| c == '}').count();

    if left_count != right_count || left_count % 2 == 0 {
        return None;
    }

    let variable = &input[left_count..input.len() - right_count];
    if variable.contains('{') || variable.contains('}') || variable.contains(' ') {
        return None;
    }

    let parts: Vec<&str> = variable.split('=').collect();
    if parts.len() != 2 || parts[0].is_empty() {
        return None;
    }

    let name = parts[0].split(':').next().unwrap().to_string();
    if name.is_empty() {
        return None;
    }
    let default = match parts[1] {
        "" => None,
        x => Some(x.to_string()),
    };
    Some(Variable { name, default })
}

/// Extracts all variables from the input string.
///
/// This function first extracts all substrings enclosed in balanced braces using `extract_braces`,
/// then attempts to parse each extracted substring as a variable using `get_variable`.
///
/// # Arguments
///
/// * `input` - A string slice that may contain variable definitions.
///
/// # Returns
///
/// A vector of `Variable` structs, each representing a successfully parsed variable from the input.
pub fn extract_variables(input: &str) -> Vec<Variable> {
    extract_braces(input)
        .into_iter()
        .filter_map(get_variable)
        .collect()
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
    fn variable_none(#[case] input: &str) {
        assert_eq!(get_variable(input), None);
    }

    #[rstest]
    #[case("{a=}")]
    #[case("{a:.2f=}")]
    fn variable_without_default(#[case] input: &str) {
        let result = get_variable(input).unwrap();
        assert_eq!(result.name, "a".to_string());
        assert_eq!(result.default, None)
    }

    #[rstest]
    #[case("{a=1.0}")]
    #[case("{a:.2f=1.0}")]
    fn variable_with_default(#[case] input: &str) {
        let result = get_variable(input).unwrap();
        assert_eq!(result.name, "a".to_string());
        assert_eq!(result.default, Some("1.0".to_string()));
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("Hello, {name}")]
    fn extract_variables_empty(#[case] input: &str) {
        assert_eq!(extract_variables(input), Vec::new());
    }

    #[test]
    fn variables_with_defaults() {
        let result = extract_variables("{name=John} is {age=30} years old.");
        assert_eq!(
            result,
            vec![
                Variable {
                    name: "name".to_string(),
                    default: Some("John".to_string())
                },
                Variable {
                    name: "age".to_string(),
                    default: Some("30".to_string())
                },
            ]
        );
    }

    #[test]
    fn variables_mixed() {
        let result =
            extract_variables("Hello, {name=}! Today is {day} and it's {temperature=20} degrees.");
        assert_eq!(
            result,
            vec![
                Variable {
                    name: "name".to_string(),
                    default: None,
                },
                Variable {
                    name: "temperature".to_string(),
                    default: Some("20".to_string())
                },
            ]
        );
    }
}
