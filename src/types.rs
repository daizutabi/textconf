use regex::Regex;

pub(crate) fn is_int(value: &str) -> bool {
    let re = Regex::new(r"^[+-]?[0-9]+$").unwrap();
    re.is_match(value)
}

pub(crate) fn is_float(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let re = Regex::new(r"^[+-]?[0-9]*\.[0-9]*$").unwrap();
    if re.is_match(value) {
        return true;
    }

    let value = value.to_lowercase();

    //         return True
}

// def is_float(x: str) -> bool:
//     if len(x) <= 1:
//         return False

//     if re.match(r"^[+-]?[0-9]*\.[0-9]*$", x):
//         return True

//     x = x.lower()
//     if x.count("e") == 1:
//         a, b = x.split("e")
//         return (is_float(a) or is_int(a)) and is_int(b)

//     return False

// def is_true(x: str) -> bool:
//     return x.lower() == "true"

// def is_false(x: str) -> bool:
//     return x.lower() == "false"

// def is_bool(x: str) -> bool:
//     return is_true(x) or is_false(x)

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
}
// import pytest

// @pytest.mark.parametrize(
//     ["x", "a"], [("", False), ("0", True), ("9", True), ("1.0", False), ("a", False)]
// )
// def test_is_int(x, a):
//     from textconf.types import is_int

//     assert is_int(x) == a
//     assert is_int(f"+{x}") == a
//     assert is_int(f"-{x}") == a
//     assert is_int(f"++{x}") is False

// @pytest.mark.parametrize(
//     ["x", "a"],
//     [
//         ("", False),
//         (".", False),
//         ("0", False),
//         ("1.0", True),
//         ("1.", True),
//         (".1", True),
//         ("a", False),
//         ("ab", False),
//         ("aeb", False),
//         ("1e1", True),
//         ("1e+1", True),
//         ("1e-1", True),
//         ("1e++1", False),
//         ("1e--1", False),
//         ("1e+-1", False),
//         ("1e1.1", False),
//         ("1.1e1", True),
//         ("1.1e1", True),
//     ],
// )
// def test_is_float(x, a):
//     from textconf.types import is_float

//     assert is_float(x) == a
//     if a:
//         assert is_float(f"+{x}") is True
//         assert is_float(f"-{x}") is True

// @pytest.mark.parametrize(
//     ["x", "a"], [("true", True), ("True", True), ("false", True), ("False", True)]
// )
// def test_is_bool(x, a):
//     from textconf.types import is_bool

//     assert is_bool(x) == a
