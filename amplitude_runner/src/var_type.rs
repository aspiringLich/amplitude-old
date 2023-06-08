use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
#[serde(into = "String", try_from = "String")]
pub enum VariableType {
    Number,
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
            VariableType::Number => "number".to_string(),
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
            return Ok(VariableType::Array(Box::new(VariableType::try_from(s)?)));
        }
        // struct
        else if s.starts_with("{") {
            anyhow::ensure!(
                s.ends_with("}"),
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
                    "Invalid character {}, Field names should only be made up of alphanumeric characters or \"_-\"",
                    ident.as_bytes()[ident.find(predicate).unwrap()] as char
                );
                map.insert(ident.to_string(), VariableType::try_from(ty)?);
            }
            Ok(VariableType::Struct(map))
        }
        // tuple
        else if s.starts_with("(") {
            anyhow::ensure!(
                s.ends_with(")"),
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
                "number" => Ok(VariableType::Number),
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
        let num = || VariableType::try_from("number").unwrap();
        let string = || VariableType::try_from("string").unwrap();
        let bool = || VariableType::try_from("bool").unwrap();
        let array = |t| VariableType::Array(Box::new(t));
        let class = |fields: &[(&'static str, VariableType)]| {
            VariableType::Struct(
                fields
                    .into_iter()
                    .cloned()
                    .map(|(a, b)| (a.to_string(), b))
                    .collect(),
            )
        };
        let tuple = |fields: &[VariableType]| VariableType::Tuple(fields.to_vec());

        assert_eq!(num(), VariableType::Number);
        assert_eq!(string(), VariableType::String);
        assert_eq!(bool(), VariableType::Boolean);

        let test = |s, t| {
            assert_eq!(
                VariableType::try_from(s).expect("No error in passed string"),
                t
            );
        };
        test("number[]", array(num()));
        test("  number[]  ", array(num()));
        test("bool[]  ", array(bool()));
        test("    \nstring[]", array(string()));
        test("number[][]", array(array(num())));

        test("{}", class(&[]));
        test("  {   }   ", class(&[]));
        test("  { test:   number,  }   ", class(&[("test", num())]));
        test(
            "  { test:   number,  test2: string,  }   ",
            class(&[("test", num()), ("test2", string())]),
        );
        test(
            "{ 1: {1: {1: {1: number}}}, 2: {}}",
            class(&[
                (
                    "1",
                    class(&[("1", class(&[("1", class(&[("1", num())]))]))]),
                ),
                ("2", class(&[])),
            ]),
        );

        test("()", tuple(&[]));
        test("(number,)", tuple(&[num()]));
        test("(number, string,)", tuple(&[num(), string()]));
        test("(number, string)", tuple(&[num(), string()]));

        Ok(())
    }
}
