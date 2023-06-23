use std::{error, fmt, str::FromStr};

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Sequence, Debug, Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
    #[serde(rename = "javascript")]
    #[serde(alias = "js")]
    JavaScript,
    #[serde(rename = "python")]
    #[serde(alias = "py")]
    Python,
    #[serde(rename = "rust")]
    #[serde(alias = "rs")]
    Rust,
    #[serde(rename = "java")]
    Java,
}

impl Language {
    pub fn image(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::Rust => "rust",
            Self::Java => "java",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::JavaScript => "js",
            Self::Python => "py",
            Self::Rust => "rs",
            Self::Java => "java",
        }
    }
}

#[derive(Debug)]
pub struct LanguageIdentError;

impl fmt::Display for LanguageIdentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not identify language")
    }
}

impl error::Error for LanguageIdentError {}

impl FromStr for Language {
    type Err = LanguageIdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "c" => Language::C,
            "c++" | "cpp" => Language::Cpp,
            "javascript" | "js" => Language::JavaScript,
            "python" | "py" => Language::Python,
            "rust" | "rs" => Language::Rust,
            "java" => Language::Java,
            _ => return Err(LanguageIdentError),
        })
    }
}
