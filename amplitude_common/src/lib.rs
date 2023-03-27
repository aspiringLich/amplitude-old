#![feature(let_chains)]
#![feature(decl_macro)]

// use std::{sync::LazyLock, path::PathBuf};
use clap::Parser;

pub mod template_builder;

// pub static ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

#[derive(Parser, Debug)]
pub struct Args {
    /// Watch the input directory for changes
    #[arg(short, long)]
    pub watch: bool,
}

pub mod config {
    use std::{
        fmt::Display,
        path::{Path, PathBuf},
    };

    /// A struct that represents a path, which can be initialized statically.
    pub struct StaticPath(&'static str);

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

    macro def_static($name:ident, $str:literal) {
        pub static $name: StaticPath = StaticPath($str);
    }

    def_static!(INPUT, "courses");
    def_static!(OUTPUT, "rendered");
    def_static!(TEMPLATE, "web/templates");
    def_static!(STATIC, "web/static");
}
