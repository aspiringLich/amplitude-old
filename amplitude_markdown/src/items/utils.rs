use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display},
    path::Path,
};

use amplitude_common::lang::Language;
use enum_iterator::all;

pub enum DirItem {
    Code { name: String, ext: String },
    Markdown { name: String },
    Toml { name: String },
    Dir { name: String },
    Other { name: String, ext: String },
}

fn os_to_str(s: &std::ffi::OsStr) -> String {
    s.to_str().unwrap().to_string()
}

pub fn get_dir_contents(path: &Path) -> Vec<DirItem> {
    let mut items = Vec::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let name = os_to_str(path.file_stem().unwrap());
        let ext = os_to_str(path.extension().unwrap_or_default());

        let code_ext = all::<Language>()
            .map(|l| l.extension().to_string())
            .collect::<HashSet<_>>();

        let item = match ext.as_str() {
            "md" => DirItem::Markdown { name },
            "toml" => DirItem::Toml { name },
            s @ _ if code_ext.contains(s) => DirItem::Code { name, ext },
            _ => DirItem::Other { name, ext },
        };
        items.push(item);
    }
    items
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
