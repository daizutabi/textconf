use regex::Regex;

pub(crate) fn is_int(value: &str) -> bool {
    let re = Regex::new(r"^[+-]?[0-9]+$").unwrap();
    re.is_match(value)
}

pub(crate) fn is_float(value: &str) -> bool {
    if value.len() <= 1 {
        return false;
    }

    let re = Regex::new(r"^[+-]?[0-9]*\.[0-9]*$").unwrap();
    if re.is_match(value) {
        return true;
    }
    let value = value.to_lowercase();
    let parts: Vec<&str> = value.split('e').collect();
    if parts.len() != 2 {
        return false;
    }

    (is_float(parts[0]) || is_int(parts[0])) && is_int(parts[1])
}

pub(crate) fn is_true(x: &str) -> bool {
    x.to_lowercase() == "true"
}

pub(crate) fn is_false(x: &str) -> bool {
    x.to_lowercase() == "false"
}

pub(crate) fn is_bool(x: &str) -> bool {
    is_true(x) || is_false(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("", false)]
    #[case("0", true)]
    #[case("9", true)]
    #[case("1.0", false)]
    #[case("a", false)]
    fn test_is_int(#[case] value: &str, #[case] result: bool) {
        assert_eq!(is_int(value), result);
        assert_eq!(is_int(format!("+{value}").as_str()), result);
        assert_eq!(is_int(format!("-{value}").as_str()), result);
        assert_eq!(is_int(format!("+-{value}").as_str()), false);
    }

    #[rstest]
    #[case("", false)]
    #[case(".", false)]
    #[case("0", false)]
    #[case("1.0", true)]
    #[case("1.", true)]
    #[case(".1", true)]
    #[case("a", false)]
    #[case("ab", false)]
    #[case("aeb", false)]
    #[case("1e1", true)]
    #[case("1e+1", true)]
    #[case("1e-1", true)]
    #[case("1e++1", false)]
    #[case("1e--1", false)]
    #[case("1e+-1", false)]
    #[case("1e1.1", false)]
    #[case("1.1e1", true)]
    #[case("1.1e1", true)]
    fn test_is_float(#[case] value: &str, #[case] result: bool) {
        assert_eq!(is_float(value), result);
        if result {
            assert_eq!(is_float(format!("+{value}").as_str()), true);
            assert_eq!(is_float(format!("-{value}").as_str()), true);
            assert_eq!(is_float(value.to_uppercase().as_str()), true);
        }
    }

    #[rstest]
    #[case("true", true)]
    #[case("TRUE", true)]
    #[case("True", true)]
    #[case("false", false)]
    #[case("FALSE", false)]
    #[case("False", false)]
    #[case("other", false)]
    fn test_is_true(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_true(input), expected);
    }

    #[rstest]
    #[case("false", true)]
    #[case("FALSE", true)]
    #[case("False", true)]
    #[case("true", false)]
    #[case("TRUE", false)]
    #[case("True", false)]
    #[case("other", false)]
    fn test_is_false(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_false(input), expected);
    }

    #[rstest]
    #[case("true", true)]
    #[case("TRUE", true)]
    #[case("True", true)]
    #[case("false", true)]
    #[case("FALSE", true)]
    #[case("False", true)]
    #[case("other", false)]
    #[case("", false)]
    fn test_is_bool(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_bool(input), expected);
    }
}
