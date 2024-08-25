use crate::params::{Parameter, Parameters};
use crate::types::{is_bool, is_float, is_int, is_true};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
enum Kind {
    Int,
    Float,
    String,
    Bool,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            &Kind::Int => "int",
            &Kind::Float => "float",
            &Kind::String => "str",
            &Kind::Bool => "bool",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    name: String,
    kind: Kind,
    default: String,
}

impl From<&Parameter> for Field {
    fn from(param: &Parameter) -> Self {
        let mut default = param.default();
        let kind = if is_bool(default) {
            default = if is_true(default) { "True" } else { "False" };
            Kind::Bool
        } else if is_int(default) {
            Kind::Int
        } else if is_float(default) {
            Kind::Float
        } else {
            Kind::String
        };

        Field {
            name: param.name().to_string(),
            kind,
            default: default.to_string(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Field {
    type Error = <Parameter as TryFrom<&'a str>>::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let param = Parameter::try_from(value)?;
        Ok((&param).into())
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::String => write!(f, "{}: {} = \"{}\"", self.name, self.kind, self.default),
            _ => write!(f, "{}: {} = {}", self.name, self.kind, self.default),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Fields {
    vec: Vec<Field>,
}

impl std::ops::Deref for Fields {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl From<&Parameters> for Fields {
    fn from(value: &Parameters) -> Self {
        let vec: Vec<_> = value.iter().map(Field::from).collect();
        Fields { vec }
    }
}

impl<'a> TryFrom<&'a str> for Fields {
    type Error = <Parameters as TryFrom<&'a str>>::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let params = Parameters::try_from(value)?;
        let vec: Vec<_> = params.iter().map(Field::from).collect();
        Ok(Fields { vec })
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields: String = self.iter().map(|f| format!("    {}\n", f)).collect();
        write!(f, "{}", fields)
    }
}

pub struct Dataclass {
    name: String,
    fields: Fields,
}

impl Dataclass {
    const TARGET: &'static str = "_target_";
}

static TARGET_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{_target_:([^}]+)\}").unwrap());

impl<'a> TryFrom<&'a str> for Dataclass {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields = Fields::try_from(value)?;
        let name = TARGET_RE
            .captures(value)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("{} field not found", Dataclass::TARGET))?;

        Ok(Dataclass { name, fields })
    }
}

impl std::fmt::Display for Dataclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cls = format!("@dataclass\nclass {}:", self.name);
        write!(f, "{}\n{}", cls, self.fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_field_from_int_parameter() {
        let field = Field::try_from("{age=30}").unwrap();
        assert_eq!(field.name, "age");
        assert_eq!(field.kind, Kind::Int);
        assert_eq!(field.default, "30");
    }

    #[test]
    fn test_field_from_float_parameter() {
        let field = Field::try_from("{price=19.99}").unwrap();
        assert_eq!(field.name, "price");
        assert_eq!(field.kind, Kind::Float);
        assert_eq!(field.default, "19.99");
    }

    #[test]
    fn test_field_from_string_parameter() {
        let field = Field::try_from("{name=John}").unwrap();
        assert_eq!(field.name, "name");
        assert_eq!(field.kind, Kind::String);
        assert_eq!(field.default, "John");
    }

    #[test]
    fn test_field_from_bool_parameter_true() {
        let field = Field::try_from("{is_active=true}").unwrap();
        assert_eq!(field.name, "is_active");
        assert_eq!(field.kind, Kind::Bool);
        assert_eq!(field.default, "True");
    }

    #[test]
    fn test_field_from_bool_parameter_false() {
        let field = Field::try_from("{is_deleted=false}").unwrap();
        assert_eq!(field.name, "is_deleted");
        assert_eq!(field.kind, Kind::Bool);
        assert_eq!(field.default, "False");
    }

    #[test]
    fn test_field_from_invalid_parameter() {
        let result = Field::try_from("{invalid}");
        assert!(result.is_err());
    }

    #[test]
    fn test_field_display_int() {
        let field = Field {
            name: "age".to_string(),
            kind: Kind::Int,
            default: "30".to_string(),
        };
        assert_eq!(field.to_string(), "age: int = 30");
    }

    #[test]
    fn test_field_display_float() {
        let field = Field {
            name: "price".to_string(),
            kind: Kind::Float,
            default: "19.99".to_string(),
        };
        assert_eq!(field.to_string(), "price: float = 19.99");
    }

    #[test]
    fn test_field_display_string() {
        let field = Field {
            name: "name".to_string(),
            kind: Kind::String,
            default: "John".to_string(),
        };
        assert_eq!(field.to_string(), "name: str = \"John\"");
    }

    #[test]
    fn test_field_display_bool() {
        let field = Field {
            name: "is_active".to_string(),
            kind: Kind::Bool,
            default: "True".to_string(),
        };
        assert_eq!(field.to_string(), "is_active: bool = True");
    }

    #[test]
    fn test_fields_from_multiple_parameters() {
        let fields =
            Fields::try_from("{count=10}{price=15.99}{name=Alice}{is_active=true}").unwrap();

        assert_eq!(fields.len(), 4);

        assert_eq!(fields[0].name, "count");
        assert_eq!(fields[0].kind, Kind::Int);
        assert_eq!(fields[0].default, "10");

        assert_eq!(fields[1].name, "price");
        assert_eq!(fields[1].kind, Kind::Float);
        assert_eq!(fields[1].default, "15.99");

        assert_eq!(fields[2].name, "name");
        assert_eq!(fields[2].kind, Kind::String);
        assert_eq!(fields[2].default, "Alice");

        assert_eq!(fields[3].name, "is_active");
        assert_eq!(fields[3].kind, Kind::Bool);
        assert_eq!(fields[3].default, "True");
    }

    #[test]
    fn test_fields_display() {
        let fields = Fields::try_from(
            "{\n{_target_:abc}{count=10}{price=15.99}{name=Alice}{is_active=true}\n}",
        )
        .unwrap();

        let expected =
            "    count: int = 10\n    price: float = 15.99\n    name: str = \"Alice\"\n    is_active: bool = True\n";
        assert_eq!(fields.to_string(), expected);
    }

    #[test]
    fn test_empty_fields() {
        let fields = Fields::try_from("").unwrap();

        assert!(fields.is_empty());
        assert_eq!(fields.to_string(), "");
    }

    #[test]
    fn test_dataclass_from_str() {
        let dataclass = Dataclass::try_from(
            "{\n{_target_:abc}{count=10}{price=15.99}{name=Alice}{is_active=true}\n}",
        )
        .unwrap();
        assert_eq!(dataclass.name, "abc");
    }

    #[test]
    fn test_dataclass_display() {
        let dataclass = Dataclass::try_from(
            "{\n{_target_:abc}{count=10}{price=15.99}{name=Alice}{is_active=true}\n}",
        )
        .unwrap();
        let expected = "@dataclass\nclass abc:\n    count: int = 10\n";
        assert!(dataclass.to_string().starts_with(expected));
    }

    #[test]
    fn test_dataclass_missing_target() {
        let result = Dataclass::try_from("{count=10}{price=15.99}{name=Alice}{is_active=true}");
        assert!(result.is_err());
    }
}
