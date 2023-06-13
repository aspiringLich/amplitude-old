#![feature(decl_macro)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(default_free_fn)]
#![feature(drain_filter)]

use std::ffi::OsStr;

// mod item;
pub mod items;
/// This crate parses markdown and toml files to build rendered html course
/// pages.
pub mod parse;

pub trait OsStrToString {
    /// I am sick and tired of `OsStr` and `Option<&OsStr>` being absolute
    /// cancer to convert to a `String`.
    fn to_string(&self) -> String;
}

impl OsStrToString for OsStr {
    fn to_string(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}

impl OsStrToString for Option<&OsStr> {
    fn to_string(&self) -> String {
        self.unwrap().to_str().unwrap().to_string()
    }
}
