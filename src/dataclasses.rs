use crate::params::{Parameter, Parameters};
use crate::types::{is_bool, is_float, is_int, is_true};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    name: String,
    kind: Kind,
    default: String,
}

impl Field {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn default(&self) -> &str {
        &self.default
    }

    pub fn remove_prefix(&mut self) -> Option<String> {
        match self.name.rsplit_once('.') {
            Some((prefix, name)) => {
                let prefix = prefix.to_string();
                self.name = name.to_string();
                Some(prefix)
            }
            None => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("missing default value: {0}")]
    MissingDefault(String),
}

impl TryFrom<&Parameter> for Field {
    type Error = FieldError;

    fn try_from(param: &Parameter) -> Result<Self, Self::Error> {
        let default = param
            .default()
            .ok_or_else(|| FieldError::MissingDefault(param.name().to_string()))?;

        let (kind, default) = match default {
            d if is_bool(d) => (
                Kind::Bool,
                if is_true(d) { "True" } else { "False" }.to_string(),
            ),
            d if is_int(d) => (Kind::Int, d.to_string()),
            d if is_float(d) => (Kind::Float, d.to_string()),
            d => (Kind::String, d.to_string()),
        };

        Ok(Field {
            name: param.name().to_string(),
            kind,
            default,
        })
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

#[derive(Debug, PartialEq, Default)]
pub struct Fields {
    vec: Vec<Field>,
}

impl std::ops::Deref for Fields {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl std::ops::DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl From<&Parameters> for Fields {
    fn from(value: &Parameters) -> Self {
        let vec: Vec<_> = value
            .iter()
            .filter_map(|param| Field::try_from(param).ok())
            .collect();
        Fields { vec }
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields: String = self.iter().map(|f| format!("    {}\n", f)).collect();
        write!(f, "{}", fields)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Dataclass {
    name: Option<String>,
    fields: Fields,
}

impl Dataclass {
    pub fn new(name: Option<&str>) -> Self {
        Dataclass {
            name: name.map(|s| s.to_string()),
            fields: Fields { vec: Vec::new() },
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Dataclasses {
    vec: Vec<Dataclass>,
}

impl std::ops::Deref for Dataclasses {
    type Target = Vec<Dataclass>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl Dataclasses {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: Option<&str>) -> Option<&Dataclass> {
        self.vec
            .iter()
            .find(|&dataclass| dataclass.name.as_deref() == name)
    }
    pub fn get_mut(&mut self, name: Option<&str>) -> Option<&mut Dataclass> {
        self.vec
            .iter_mut()
            .find(|dataclass| dataclass.name.as_deref() == name)
    }

    pub fn push(&mut self, mut field: Field) {
        let prefix = field.remove_prefix();
        if let Some(dataclass) = self.get_mut(prefix.as_deref()) {
            dataclass.fields.push(field);
        } else {
            let dataclass = Dataclass::new(prefix.as_deref());
            self.vec.push(dataclass);
        }
    }
}

// impl<'a> TryFrom<&'a str> for Dataclass {
//     type Error = anyhow::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let fields = Fields::try_from(value)?;
//         let name = TARGET_RE
//             .captures(value)
//             .and_then(|caps| caps.get(1))
//             .map(|m| m.as_str().to_string())
//             .ok_or_else(|| anyhow::anyhow!("{} field not found", Dataclass::TARGET))?;

//         Ok(Dataclass { name, fields })
//     }
// }

// impl std::fmt::Display for Dataclass {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let cls = format!("@dataclass\nclass {}:", self.name);
//         write!(f, "{}\n{}", cls, self.fields)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn test_field_remove_prefix() {
        let mut field = Field {
            name: "user.profile.age".to_string(),
            kind: Kind::Int,
            default: "25".to_string(),
        };
        let prefix = field.remove_prefix();
        assert_eq!(field.name, "age");
        assert_eq!(prefix, Some("user.profile".to_string()));
    }

    #[test]
    fn test_field_remove_prefix_no_prefix() {
        let mut field = Field {
            name: "age".to_string(),
            kind: Kind::Int,
            default: "25".to_string(),
        };
        let prefix = field.remove_prefix();
        assert_eq!(field.name, "age");
        assert_eq!(prefix, None);
    }

    #[test]
    fn test_dataclasses_get() {
        let field = Field {
            name: "age".to_string(),
            kind: Kind::Int,
            default: "25".to_string(),
        };
        let dataclasses = Dataclasses {
            vec: vec![
                Dataclass {
                    name: Some("user".to_string()),
                    fields: Fields {
                        vec: vec![field.clone()],
                    },
                },
                Dataclass {
                    name: None,
                    fields: Fields { vec: vec![field] },
                },
            ],
        };
        assert_eq!(dataclasses.get(Some("user")), Some(&dataclasses[0]));
        assert_eq!(dataclasses.get(None), Some(&dataclasses[1]));
    }

    #[test]
    fn test_dataclasses_push() {
        let mut dataclasses = Dataclasses::new();
        let field = Field {
            name: "age".to_string(),
            kind: Kind::Int,
            default: "25".to_string(),
        };
        dataclasses.push(field);
        assert_eq!(dataclasses.get(None), Some(&dataclasses[0]));
    }

    #[test]
    fn test_dataclasses_push_with_prefix() {
        let mut dataclasses = Dataclasses::new();
        let field = Field {
            name: "user.profile.age".to_string(),
            kind: Kind::Int,
            default: "25".to_string(),
        };
        dataclasses.push(field);
        assert_eq!(dataclasses.get(Some("user.profile")), Some(&dataclasses[0]));
    }
}
