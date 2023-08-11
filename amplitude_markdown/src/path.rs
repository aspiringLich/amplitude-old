use crate::{parse::context::DataContext, OsStrToString};
use amplitude_common::config::Config;
use amplitude_runner::lang::Language;
use anyhow::Context;
use enum_iterator::all;
use std::{
    fmt::{self, Display},
    fs::File,
    path::{Path, PathBuf},
};

pub trait FromFile {
    fn from_file(
        filename: &str,
        file: &mut File,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait FromDirectory
where
    Self: Sized,
{
    fn from_directory(
        content: &DirectoryContent,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<Self>;
}

pub fn from_directory<T: FromDirectory>(
    path: &Path,
    context: &mut DataContext,
    cfg: &Config,
) -> anyhow::Result<T> {
    let content: DirectoryContent<'_> = DirectoryContent::new(path)?;
    T::from_directory(&content, context, cfg)
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

#[derive(Debug, Clone)]
pub struct DirItem<'a> {
    pub name: String,
    pub ext: String,
    pub item_type: FileType,
    pub parent_path: &'a Path,
}

impl<'a> DirItem<'a> {
    pub fn path(&self) -> PathBuf {
        self.parent_path.join(&self.name).with_extension(&self.ext)
    }

    pub fn read_to_string(&self) -> anyhow::Result<String> {
        std::fs::read_to_string(self.path())
            .with_context(|| format!("While reading file `{}`", self.path().display()))
    }
}

pub struct DirectoryContent<'a> {
    pub directories: Vec<String>,
    files: Vec<DirItem<'a>>,
    pub path: &'a Path,
}

impl<'a> DirectoryContent<'a> {
    /// Get a file by name and filetype, erroring if it doesn't exist or if there are multiple
    /// that match the query.
    pub fn query_file(&'a self, name: &'a str, typ: FileType) -> anyhow::Result<&'a DirItem> {
        let mut iter = self
            .files
            .iter()
            .filter(move |item| item.item_type == typ)
            .filter(move |item| item.name == name)
            .peekable();
        let burner = iter.next();
        if burner.is_none() {
            anyhow::bail!("Required file `{}.{}` not found!", name, typ);
        }
        if iter.peek().is_some() {
            anyhow::bail!(
                "Multiple files `{}.{}` found! Expected only one...",
                name,
                typ
            );
        }
        Ok(burner.unwrap())
    }

    /// Get files by name and filetype, erroring if they don't exist.
    pub fn query_files(
        &'a self,
        name: &'a str,
        typ: FileType,
    ) -> anyhow::Result<impl Iterator<Item = &'a DirItem>> {
        let mut iter = self
            .files
            .iter()
            .filter(move |item| item.item_type == typ)
            .filter(move |item| item.name == name)
            .peekable();
        if iter.peek().is_none() {
            anyhow::bail!("Required file(s) `{}.{}` not found!", name, typ);
        }
        Ok(iter)
    }

    pub fn query_filetype(&self, typ: FileType) -> anyhow::Result<impl Iterator<Item = &DirItem>> {
        let mut iter = self
            .files
            .iter()
            .filter(move |item| item.item_type == typ)
            .peekable();
        if iter.peek().is_none() {
            anyhow::bail!("Required file of type `{}` not found!", typ);
        }
        Ok(iter)
    }

    pub fn get_directory(&self, name: &str) -> anyhow::Result<PathBuf> {
        self.directories
            .iter()
            .find(|&d| d == name)
            .map(|s| self.path.join(s))
            .ok_or_else(|| anyhow::anyhow!("Required directory `{}` not found!", name))
    }

    pub fn new(path: &'a Path) -> anyhow::Result<Self> {
        let mut files = Vec::new();
        let mut directories = Vec::new();

        for entry in path.read_dir()? {
            let entry = entry.unwrap();
            let item_path = entry.path();

            let name = item_path.file_stem().unwrap().to_string();
            let ext = item_path.extension().unwrap_or_default().to_string();

            let item_type = if item_path.is_dir() {
                // files.extend(Self::new(&path)?.files.drain(..).map(|p| DirItem {
                //     name: format!("{}/{}", name, p.name),
                //     ..p
                // }));
                directories.push(name);
                continue;
            } else {
                FileType::from_ext(&ext)
            };

            files.push(DirItem {
                name,
                ext,
                item_type,
                parent_path: path,
            });
        }
        Ok(DirectoryContent {
            directories,
            files,
            path,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Code,
    Markdown,
    Toml,
    Other,
}

impl Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FileType::Code => "<code_ext>",
            FileType::Markdown => "md",
            FileType::Toml => "toml",
            FileType::Other => "*",
        })
    }
}
