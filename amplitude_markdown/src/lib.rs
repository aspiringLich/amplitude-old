#![feature(decl_macro)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(default_free_fn)]

// mod item;
/// This crate parses markdown and toml files to build rendered html course
/// pages.
pub mod parse;
pub mod items;