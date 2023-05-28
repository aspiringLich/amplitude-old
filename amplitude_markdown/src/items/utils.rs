use std::{
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
    path::Path,
};

use amplitude_common::lang::Language;
use enum_iterator::all;

pub struct DirContents {
    contents: Vec<DirItem>,
}

impl Deref for DirContents {
    type Target = Vec<DirItem>;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl DerefMut for DirContents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contents
    }
}

impl DirContents {
    pub fn query<'a>(&'a self, name: &'a str, typ: FileType) -> impl Iterator<Item = &'a DirItem> {
        self.iter()
            .filter(move |item| item.item_type == typ)
            .filter(move |item| item.name == name)
    }

    pub fn query_type(&self, typ: FileType) -> impl Iterator<Item = &DirItem> {
        self.iter().filter(move |item| item.item_type == typ)
    }

    pub fn contains(&self, path: &str) -> bool {
        let Some((name, ext)) = path.split_once(".") else { return false };
        self.iter().any(|item| item.name == name && item.ext == ext)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Code,
    Markdown,
    Toml,
    Directory,
    Other,
}

impl Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FileType::Code => "<code_ext>",
            FileType::Markdown => "md",
            FileType::Toml => "toml",
            FileType::Directory => "<directory>",
            FileType::Other => "*",
        })
    }
}

#[ctor::ctor]
static CODE_EXT: Vec<&'static str> = all::<Language>().map(|l| l.extension()).collect();

impl FileType {
    pub fn from_ext(ext: &str) -> Self {
        match ext {
            "md" => FileType::Markdown,
            "toml" => FileType::Toml,
            _ if CODE_EXT.contains(&ext) => FileType::Code,
            _ => FileType::Other,
        }
    }
}

pub struct DirItem {
    pub name: String,
    pub ext: String,
    pub item_type: FileType,
}

fn os_to_str(s: &std::ffi::OsStr) -> String {
    s.to_str().unwrap().to_string()
}

pub fn get_dir_contents(path: &Path) -> anyhow::Result<DirContents> {
    let mut items = Vec::new();
    for entry in path.read_dir()? {
        let entry = entry.unwrap();
        let path = entry.path();

        let name = os_to_str(path.file_stem().unwrap());
        let ext = os_to_str(path.extension().unwrap_or_default());

        let item_type = if path.is_dir() {
            FileType::Directory
        } else {
            FileType::from_ext(&ext)
        };

        items.push(DirItem {
            name,
            ext,
            item_type,
        });
    }
    Ok(DirContents { contents: items })
}

#[derive(Default, Debug)]
pub struct ErrorList<T: Display + Debug> {
    errors: Vec<T>,
    initial: String,
    stop: &'static str,
}

impl<T: Display + Debug> ErrorList<T> {
    pub fn new(initial: impl Display, stop: &'static str) -> Self {
        Self {
            errors: Vec::new(),
            initial: initial.to_string(),
            stop,
        }
    }

    pub fn push(&mut self, err: T) {
        self.errors.push(err);
    }
}

fn indent<F>(mut f: F, s: impl Display) -> String
where
    F: FnMut(usize, bool) -> String,
{
    let s = s.to_string();
    let lines = s.lines().collect::<Vec<_>>();
    lines
        .iter()
        .enumerate()
        .map(|(i, s)| f(i, i == lines.len() - 1) + &s)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

impl Display for ErrorList<anyhow::Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.initial)?;

        let indent_n = |i, s| {
            indent(
                |n: usize, last| {
                    if n == 0 {
                        format!("{i}: ")
                    } else if last {
                        "└  ".to_string()
                    } else {
                        "│  ".to_string()
                    }
                },
                s,
            )
        };
        // let indent = |s| indent(|_, _| "│ ".to_string(), s);

        for (i, err) in self.errors.iter().enumerate() {
            let mut out = String::new();
            out += &format!("{err}\n");

            let chain = err.chain().collect::<Vec<_>>();
            if chain.len() > 1 {
                out += "\nCaused By:\n";
                for (i, err) in chain.into_iter().skip(1).enumerate() {
                    out += &format!("   {}", indent_n(i, err.to_string()));
                }
            }

            out += "\nBacktrace:\n";
            let s = err.backtrace().to_string();
            let lines = s.lines().collect::<Vec<_>>();
            let backtrace = lines
                .as_slice()
                .windows(3)
                .skip_while(|l| !l[1].contains(module_path!().split("::").next().unwrap()))
                .take_while(|l| !l[0].contains(&self.stop))
                .map(|l| l[1])
                .collect::<Vec<_>>()
                .join("\n");
            out += &backtrace;

            write!(f, "\n{}", indent_n(i, out))?;
        }

        Ok(())
    }
}

impl Error for ErrorList<anyhow::Error> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
