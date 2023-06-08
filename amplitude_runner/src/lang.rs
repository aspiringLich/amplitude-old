use std::str::FromStr;

use enum_iterator::Sequence;
use serde::Serialize;

#[derive(Sequence, Debug, Serialize, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
    #[serde(rename = "js")]
    JavaScript,
    #[serde(rename = "py")]
    Python,
    #[serde(rename = "rs")]
    Rust,
    #[serde(rename = "java")]
    Java,
}

impl Language {
    pub fn image(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::Cpp => "c++",
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

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "c" => Language::C,
            "c++" | "cpp" => Language::Cpp,
            "javascript" | "js" => Language::JavaScript,
            "python" | "py" => Language::Python,
            "rust" | "rs" => Language::Rust,
            "java" => Language::Java,
            _ => return Err(format!("Could not identify `{s}`")),
        })
    }
}
