use std::{collections::HashMap, fmt};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(into = "String", try_from = "String")]
pub enum VariableType {
    Integer,
    Float,
    String,
    Boolean,
    Array(Box<VariableType>),
    Struct(HashMap<String, VariableType>),
}

impl From<VariableType> for String {
    fn from(value: VariableType) -> Self {
        String::from(&value)
    }
}

impl<'a> From<&'a VariableType> for String {
    fn from(value: &'a VariableType) -> Self {
        match value {
            VariableType::Integer => "int".to_string(),
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
            for field in s.split(',') {
                let (mut ident, ty) = field
                    .split_once(':')
                    .context("Expected `:` inside struct/class field")?;
                ident = ident.trim();
                anyhow::ensure!(
                    !ident.contains(|c: char| c.is_ascii_alphanumeric() || "_-".contains(c)),
                    "Field names should only be made up of alphanumeric characters or \"_-\""
                );
                map.insert(ident.to_string(), VariableType::try_from(ty)?);
            }
            Ok(VariableType::Struct(map))
        } else {
            match s {
                "int" => Ok(VariableType::Integer),
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

#[cfg(tests)]
mod test {
    use super::*;
    
    
}