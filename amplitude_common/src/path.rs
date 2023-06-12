use std::{
    env,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

macro def_static($name:ident, $str:literal) {
    pub static $name: StaticPath = StaticPath($str);
}

def_static!(DATABASE, "./data.db");
def_static!(LANGUAGES, "./languages");

/// A struct that represents a path, which can be initialized statically.
#[derive(Clone, Default)]
pub struct StaticPath(&'static str);

pub fn scope_dir<P: AsRef<Path>, F: FnOnce() -> ()>(path: P, f: F) -> io::Result<()> {
    env::set_current_dir(path)?;
    f();
    env::set_current_dir("..")
}

impl Display for StaticPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl From<StaticPath> for PathBuf {
    fn from(path: StaticPath) -> Self {
        PathBuf::from(path.0)
    }
}

impl From<StaticPath> for &Path {
    fn from(path: StaticPath) -> Self {
        Path::new(path.0)
    }
}

impl AsRef<Path> for StaticPath {
    fn as_ref(&self) -> &Path {
        Path::new(self.0)
    }
}

impl StaticPath {
    /// Returns the path as a `Path` struct.
    pub fn as_path(&self) -> &'static Path {
        Path::new(self.0)
    }

    /// Joins the path with another path.
    pub fn join<P: AsRef<Path>>(&self, other: P) -> PathBuf {
        self.as_path().join(other)
    }
}
