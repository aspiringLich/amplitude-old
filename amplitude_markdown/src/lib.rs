#![feature(decl_macro)]
/// This crate parses markdown and toml files to build rendered html course
/// pages.
pub mod parse;
mod link_concat;
mod inject;
