use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display},
    fs::File,
};

use amplitude_common::config::Config;
use anyhow::Context;

use crate::{
    parse::context::DataContext,
    path::{from_directory, DirectoryContent, FromDirectory, FromFile},
    OsStrToString,
};

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

fn indent(s: impl Display, first: impl Display, mid: impl Display, last: impl Display) -> String {
    let s = s.to_string();
    let lines = s.lines().collect::<Vec<_>>();
    lines
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let before = if i == 0 {
                first.to_string()
            } else if i < lines.len() - 1 {
                mid.to_string()
            } else {
                last.to_string()
            };
            before + s
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

impl Display for ErrorList<anyhow::Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.initial)?;

        let indent_n = |i, n: usize, s| {
            indent(
                s,
                format!("{}{i}: ", " ".repeat(n)),
                format!("{}│  ", " ".repeat(n)),
                format!("{}└  ", " ".repeat(n)),
            )
        };

        for (i, err) in self.errors.iter().enumerate() {
            let mut out = String::new();
            out += &format!("{err}\n");

            let chain = err.chain().collect::<Vec<_>>();
            if chain.len() > 1 {
                out += "\nCaused By:\n";
                for (i, err) in chain.into_iter().skip(1).enumerate() {
                    let err = err.to_string();
                    out += &indent_n(i, 3, err);
                }
            }

            out += "\nBacktrace:\n";
            let s = err.backtrace().to_string();
            let lines = s.lines().collect::<Vec<_>>();
            let backtrace = lines
                .as_slice()
                .windows(3)
                .skip_while(|l: &&[&str]| !l[1].contains(": amplitude"))
                .take_while(|l| !l[0].contains(self.stop))
                .map(|l| l[1])
                .collect::<Vec<_>>()
                .join("\n");
            out += &backtrace;

            write!(f, "\n{}", indent_n(i, 0, out))?;
        }

        Ok(())
    }
}

impl Error for ErrorList<anyhow::Error> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// A struct that expects a directory structure like this:
/// ```plaintext
/// 00-one
/// 01-two
/// 02-three
/// 03-four
/// ```
pub struct OrderedDirectories<T> {
    pub items: HashMap<String, T>,
}

impl<T> FromDirectory for OrderedDirectories<T>
where
    T: FromDirectory,
{
    fn from_directory(
        content: &DirectoryContent,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<Self> {
        let mut items = HashMap::new();

        for dir in &content.directories {
            let id = &dir.as_str()[3..];
            let path = content.path.join(&dir);

            if dir.starts_with('.') {
                continue;
            }

            items.insert(
                id.to_string(),
                from_directory(&path, context, cfg)
                    .with_context(|| format!("While parsing path {}", path.display()))?,
            );
        }

        Ok(Self { items })
    }
}

/// A struct that expects a directory structure like this:
/// ```plaintext
/// 00-one.md
/// 01-two.md
/// 02-three.md
/// 03-four.md
/// ```
pub struct OrderedFiles<T> {
    pub items: HashMap<String, T>,
}

impl<T> FromDirectory for OrderedFiles<T>
where
    T: FromFile,
{
    fn from_directory(
        content: &DirectoryContent,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<Self> {
        let mut items = HashMap::new();

        for dir in &content.directories {
            let id = &dir.as_str()[3..];
            let path = content.path.join(&dir);
            items.insert(
                id.to_string(),
                T::from_file(
                    &path
                        .file_name()
                        .context("While getting filename")?
                        .to_string(),
                    &mut File::open(&path).context("While opening file")?,
                    context,
                    cfg,
                )
                .with_context(|| format!("While parsing path {}", path.display()))?,
            );
        }

        Ok(Self { items })
    }
}
