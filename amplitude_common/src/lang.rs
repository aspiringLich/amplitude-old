use enum_iterator::Sequence;

#[derive(Sequence, Debug)]
pub enum Language {
    C,
    Cpp,
    JavaScript,
    Python,
    Rust,
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

    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "c" => Language::C,
            "c++" | "cpp" => Language::Cpp,
            "javascript" => Language::JavaScript,
            "python" => Language::Python,
            "rust" => Language::Rust,
            "java" => Language::Java,
            _ => return None,
        })
    }
}
