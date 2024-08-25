use std::sync::LazyLock;

use regex::Regex;

static INT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[+-]?[0-9]+$").unwrap());
static FLOAT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[+-]?[0-9]*\.[0-9]*$").unwrap());

pub fn is_int(value: &str) -> bool {
    INT_RE.is_match(value)
}

pub fn is_float(value: &str) -> bool {
    if value.len() <= 1 {
        return false;
    }
    if FLOAT_RE.is_match(value) {
        return true;
    }
    let value = value.to_lowercase();
    let parts: Vec<&str> = value.split('e').collect();
    if parts.len() != 2 {
        return false;
    }

    (is_float(parts[0]) || is_int(parts[0])) && is_int(parts[1])
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
}
