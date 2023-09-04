use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
#[serde(into = "String", try_from = "String")]
pub enum VariableType {
    Int,
    Float,
    String,
    Boolean,
    Array(Box<VariableType>),
    Struct(HashMap<String, VariableType>),
    Tuple(Vec<VariableType>),
}

impl From<VariableType> for String {
    fn from(value: VariableType) -> Self {
        String::from(&value)
    }
}

impl<'a> From<&'a VariableType> for String {
    fn from(value: &'a VariableType) -> Self {
        match value {
            VariableType::Int => "int".to_string(),
            VariableType::Float => "float".to_string(),
            VariableType::String => "string".to_string(),
            VariableType::Boolean => "bool".to_string(),
            VariableType::Array(ty) => format!("{}[]", ty),
            VariableType::Struct(fields) => {
                let mut out = String::new();
                for (name, ty) in fields {
                    out += &format!("{}: {},", name, ty);
                }
                format!("{{{}}}", out)
            }
            VariableType::Tuple(fields) => {
                let mut out = String::new();
                for ty in fields {
                    out += &format!("{},", ty);
                }
                format!("({})", out)
            }
        }
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.into();
        f.write_str(&s)
    }
}

impl<'a> TryFrom<&'a str> for VariableType {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let s = value.trim();

        // array
        if let Some(s) = s.strip_suffix("[]") {
            Ok(VariableType::Array(Box::new(VariableType::try_from(s)?)))
        }
        // struct
        else if s.starts_with('{') {
            anyhow::ensure!(
                s.ends_with('}'),
                "Expected ending `}}` when starting with `{{`"
            );
            let s = &s[1..s.len() - 1];
            let mut map = HashMap::new();
            let mut iter = s.split(',').peekable();

            while let Some(field) = iter.next() {
                let field = field.split_once(':');
                if field.is_none() {
                    if iter.peek().is_some() {
                        anyhow::bail!("Expected `:` in struct field");
                    } else {
                        break;
                    }
                }

                let (mut ident, ty) = field.unwrap();
                ident = ident.trim();
                let predicate = |c: char| !c.is_ascii_alphanumeric() && !"_-".contains(c);
                anyhow::ensure!(
                    !ident.contains(predicate),
                    "Invalid character {}, Field names should only be made up of alphanumeric \
                     characters or \"_-\"",
                    ident.as_bytes()[ident.find(predicate).unwrap()] as char
                );
                map.insert(ident.to_string(), VariableType::try_from(ty)?);
            }
            Ok(VariableType::Struct(map))
        }
        // tuple
        else if s.starts_with('(') {
            anyhow::ensure!(
                s.ends_with(')'),
                "Expected ending `)` when starting with `(`"
            );
            let s = &s[1..s.len() - 1];
            let mut vec = Vec::new();

            let mut iter = s.split(',').peekable();
            while let Some(ty) = iter.next() {
                if ty.trim().is_empty() {
                    if iter.peek().is_some() {
                        anyhow::bail!("Found empty tuple field");
                    } else {
                        break;
                    }
                }
                vec.push(VariableType::try_from(ty)?);
            }
            Ok(VariableType::Tuple(vec))
        } else {
            match s {
                "int" => Ok(VariableType::Int),
                "float" => Ok(VariableType::Float),
                "string" => Ok(VariableType::String),
                "bool" => Ok(VariableType::Boolean),
                _ => anyhow::bail!("Could not interpret type"),
            }
        }
    }
}

impl TryFrom<String> for VariableType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_variable_type() -> anyhow::Result<()> {
        let int = || VariableType::try_from("int").unwrap();
        let float = || VariableType::try_from("float").unwrap();
        let string = || VariableType::try_from("string").unwrap();
        let bool = || VariableType::try_from("bool").unwrap();
        let array = |t| VariableType::Array(Box::new(t));
        let class = |fields: &[(&'static str, VariableType)]| {
            VariableType::Struct(
                fields
                    .iter()
                    .cloned()
                    .map(|(a, b)| (a.to_string(), b))
                    .collect(),
            )
        };
        let tuple = |fields: &[VariableType]| VariableType::Tuple(fields.to_vec());

        assert_eq!(int(), VariableType::Int);
        assert_eq!(float(), VariableType::Float);
        assert_eq!(string(), VariableType::String);
        assert_eq!(bool(), VariableType::Boolean);

        let test = |s, t| {
            assert_eq!(
                VariableType::try_from(s).expect("No error in passed string"),
                t
            );
        };
        test("int[]", array(int()));
        test("  int[]  ", array(int()));
        test("bool[]  ", array(bool()));
        test("    \nstring[]", array(string()));
        test("int[][]", array(array(int())));

        test("{}", class(&[]));
        test("  {   }   ", class(&[]));
        test("  { test:   int,  }   ", class(&[("test", int())]));
        test(
            "  { test:   int,  test2: string,  }   ",
            class(&[("test", int()), ("test2", string())]),
        );
        test(
            "{ 1: {1: {1: {1: int}}}, 2: {}}",
            class(&[
                (
                    "1",
                    class(&[("1", class(&[("1", class(&[("1", int())]))]))]),
                ),
                ("2", class(&[])),
            ]),
        );

        test("()", tuple(&[]));
        test("(int,)", tuple(&[int()]));
        test("(int, string,)", tuple(&[int(), string()]));
        test("(int, string)", tuple(&[int(), string()]));

        Ok(())
    }
}
