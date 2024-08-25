use crate::params::{Parameter, ParameterReplacer};
use crate::types::{is_float, is_int};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Int,
    Float,
    String,
    Bool,
    List(Box<Kind>),
    Class(String),
}

impl From<&str> for Kind {
    fn from(default: &str) -> Self {
        match default {
            d if is_int(d) => Kind::Int,
            d if is_float(d) => Kind::Float,
            d => {
                if d == "True" || d == "False" {
                    Kind::Bool
                } else if d.starts_with('[') && d.ends_with(']') {
                    let inner = &d[1..d.len() - 1];
                    Kind::List(Box::new(Kind::from(inner)))
                } else {
                    Kind::String
                }
            }
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Int => write!(f, "int"),
            Kind::Float => write!(f, "float"),
            Kind::String => write!(f, "str"),
            Kind::Bool => write!(f, "bool"),
            Kind::List(ref k) => write!(f, "list[{}]", k),
            Kind::Class(ref name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    name: String,
    kind: Kind,
    default: String,
}

#[derive(Error, Debug)]
pub enum FieldError {
    #[error("Parameter default is empty: found {0}")]
    EmptyDefault(String),
}

impl<'a> TryFrom<&'a Parameter<'a>> for Field {
    type Error = FieldError;

    fn try_from(param: &'a Parameter<'a>) -> Result<Self, Self::Error> {
        let Some(default) = param.default() else {
            return Err(FieldError::EmptyDefault(param.content().to_string()));
        };
        let name = param.name().to_string();
        let kind = Kind::from(default);
        let default = default.to_string();
        Ok(Field {
            name,
            kind,
            default,
        })
    }
}

pub struct FieldList {
    source: String,
    fields: Vec<Field>,
}

impl FieldList {
    pub fn new(input: &str, prefix: &str) -> Result<Self, FieldError> {
        let replacer = ParameterReplacer::new(input);
        let source = replacer.replace(prefix);
        let fields = replacer
            .parameters_with_default()
            .into_iter()
            .map(|p| Field::try_from(p))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FieldList { source, fields })
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

// impl Field {
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//     pub fn default(&self) -> &str {
//         &self.default
//     }

//     pub fn remove_prefix(&mut self) -> Option<String> {
//         match self.name.rsplit_once('.') {
//             Some((prefix, name)) => {
//                 let prefix = prefix.to_string();
//                 self.name = name.to_string();
//                 Some(prefix)
//             }
//             None => None,
//         }
//     }
// }

// impl std::fmt::Display for Field {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.kind {
//             Kind::String => write!(f, "{}: {} = \"{}\"", self.name, self.kind, self.default),
//             _ => write!(f, "{}: {} = {}", self.name, self.kind, self.default),
//         }
//     }
// }

// #[derive(Debug, PartialEq, Default)]
// pub struct Fields {
//     vec: Vec<Field>,
// }

// impl std::ops::Deref for Fields {
//     type Target = Vec<Field>;

//     fn deref(&self) -> &Self::Target {
//         &self.vec
//     }
// }

// impl std::ops::DerefMut for Fields {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.vec
//     }
// }

// impl From<&Parameters> for Fields {
//     fn from(value: &Parameters) -> Self {
//         let vec: Vec<_> = value
//             .filter_map(|param| Field::try_from(param).ok())
//             .collect();
//         Fields { vec }
//     }
// }

// impl std::fmt::Display for Fields {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let fields: String = self.iter().map(|f| format!("    {}\n", f)).collect();
//         write!(f, "{}", fields)
//     }
// }

// #[derive(Debug, PartialEq, Default)]
// pub struct Dataclass {
//     name: Option<String>,
//     fields: Fields,
// }

// impl Dataclass {
//     pub fn new(name: Option<&str>) -> Self {
//         Dataclass {
//             name: name.map(|s| s.to_string()),
//             fields: Fields { vec: Vec::new() },
//         }
//     }
// }

// #[derive(Debug, PartialEq, Default)]
// pub struct Dataclasses {
//     vec: Vec<Dataclass>,
// }

// impl std::ops::Deref for Dataclasses {
//     type Target = Vec<Dataclass>;

//     fn deref(&self) -> &Self::Target {
//         &self.vec
//     }
// }

// impl Dataclasses {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn get(&self, name: Option<&str>) -> Option<&Dataclass> {
//         self.vec
//             .iter()
//             .find(|&dataclass| dataclass.name.as_deref() == name)
//     }
//     pub fn get_mut(&mut self, name: Option<&str>) -> Option<&mut Dataclass> {
//         self.vec
//             .iter_mut()
//             .find(|dataclass| dataclass.name.as_deref() == name)
//     }

//     pub fn push(&mut self, mut field: Field) {
//         let prefix = field.remove_prefix();
//         if let Some(dataclass) = self.get_mut(prefix.as_deref()) {
//             dataclass.fields.push(field);
//         } else {
//             let dataclass = Dataclass::new(prefix.as_deref());
//             self.vec.push(dataclass);
//         }
//     }
// }

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
mod kind_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_kind_from_int() {
        assert_eq!(Kind::from("42"), Kind::Int);
    }

    #[test]
    fn test_kind_from_float() {
        assert_eq!(Kind::from("3.14"), Kind::Float);
        assert_eq!(Kind::from("3.14e-10"), Kind::Float);
    }

    #[test]
    fn test_kind_from_bool_true() {
        assert_eq!(Kind::from("True"), Kind::Bool);
    }

    #[test]
    fn test_kind_from_bool_false() {
        assert_eq!(Kind::from("False"), Kind::Bool);
    }

    #[test]
    fn test_kind_from_list() {
        assert_eq!(Kind::from("[42]"), Kind::List(Box::new(Kind::Int)));
    }

    #[test]
    fn test_kind_from_string() {
        assert_eq!(Kind::from("hello"), Kind::String);
    }

    #[test]
    fn test_kind_display_int() {
        assert_eq!(Kind::Int.to_string(), "int");
    }

    #[test]
    fn test_kind_display_float() {
        assert_eq!(Kind::Float.to_string(), "float");
    }

    #[test]
    fn test_kind_display_string() {
        assert_eq!(Kind::String.to_string(), "str");
    }

    #[test]
    fn test_kind_display_bool() {
        assert_eq!(Kind::Bool.to_string(), "bool");
    }

    #[test]
    fn test_kind_display_list() {
        assert_eq!(Kind::List(Box::new(Kind::Int)).to_string(), "list[int]");
    }

    #[test]
    fn test_kind_display_class() {
        assert_eq!(Kind::Class("MyClass".to_string()).to_string(), "MyClass");
    }

    #[test]
    fn test_field_try_from_parameter() {
        let input = "{a}{b=2}{c:.2f=3.0}";
        let replacer = ParameterReplacer::new(input);
        let params = replacer.parameters_with_default();
        let field = Field::try_from(params[0]).unwrap();
        assert_eq!(field.name, "b");
        assert_eq!(field.kind, Kind::Int);
        assert_eq!(field.default, "2");
        let field = Field::try_from(params[1]).unwrap();
        assert_eq!(field.name, "c");
        assert_eq!(field.kind, Kind::Float);
        assert_eq!(field.default, "3.0");
    }

    #[test]
    fn test_field_list_new() {
        let input = "{a}{b=2}{c.d:.2f=3.0}";
        let source = "{a}{p.b}{p.c.d:.2f}";
        let field_list = FieldList::new(input, "p.").unwrap();
        assert_eq!(field_list.source(), source);
        assert_eq!(field_list.fields().len(), 2);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_field_display_int() {
//         let field = Field {
//             name: "age".to_string(),
//             kind: Kind::Int,
//             default: "30".to_string(),
//         };
//         assert_eq!(field.to_string(), "age: int = 30");
//     }

//     #[test]
//     fn test_field_display_float() {
//         let field = Field {
//             name: "price".to_string(),
//             kind: Kind::Float,
//             default: "19.99".to_string(),
//         };
//         assert_eq!(field.to_string(), "price: float = 19.99");
//     }

//     #[test]
//     fn test_field_display_string() {
//         let field = Field {
//             name: "name".to_string(),
//             kind: Kind::String,
//             default: "John".to_string(),
//         };
//         assert_eq!(field.to_string(), "name: str = \"John\"");
//     }

//     #[test]
//     fn test_field_display_bool() {
//         let field = Field {
//             name: "is_active".to_string(),
//             kind: Kind::Bool,
//             default: "True".to_string(),
//         };
//         assert_eq!(field.to_string(), "is_active: bool = True");
//     }

//     #[test]
//     fn test_field_remove_prefix() {
//         let mut field = Field {
//             name: "user.profile.age".to_string(),
//             kind: Kind::Int,
//             default: "25".to_string(),
//         };
//         let prefix = field.remove_prefix();
//         assert_eq!(field.name, "age");
//         assert_eq!(prefix, Some("user.profile".to_string()));
//     }

//     #[test]
//     fn test_field_remove_prefix_no_prefix() {
//         let mut field = Field {
//             name: "age".to_string(),
//             kind: Kind::Int,
//             default: "25".to_string(),
//         };
//         let prefix = field.remove_prefix();
//         assert_eq!(field.name, "age");
//         assert_eq!(prefix, None);
//     }

//     #[test]
//     fn test_dataclasses_get() {
//         let field = Field {
//             name: "age".to_string(),
//             kind: Kind::Int,
//             default: "25".to_string(),
//         };
//         let dataclasses = Dataclasses {
//             vec: vec![
//                 Dataclass {
//                     name: Some("user".to_string()),
//                     fields: Fields {
//                         vec: vec![field.clone()],
//                     },
//                 },
//                 Dataclass {
//                     name: None,
//                     fields: Fields { vec: vec![field] },
//                 },
//             ],
//         };
//         assert_eq!(dataclasses.get(Some("user")), Some(&dataclasses[0]));
//         assert_eq!(dataclasses.get(None), Some(&dataclasses[1]));
//     }

//     #[test]
//     fn test_dataclasses_push() {
//         let mut dataclasses = Dataclasses::new();
//         let field = Field {
//             name: "age".to_string(),
//             kind: Kind::Int,
//             default: "25".to_string(),
//         };
//         dataclasses.push(field);
//         assert_eq!(dataclasses.get(None), Some(&dataclasses[0]));
//     }

//     #[test]
//     fn test_dataclasses_push_with_prefix() {
//         let mut dataclasses = Dataclasses::new();
//         let field = Field {
//             name: "user.profile.age".to_string(),
//             kind: Kind::Int,
//             default: "25".to_string(),
//         };
//         dataclasses.push(field);
//         assert_eq!(dataclasses.get(Some("user.profile")), Some(&dataclasses[0]));
//     }
// }
