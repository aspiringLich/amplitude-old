use afire::{prelude::*, extension::ServeStatic};
use amplitude_common::config;
use std::{fmt::Display, fs::File};
use crate::error::*;

mod course;

pub fn attach<T: Send + Sync>(server: &mut Server<T>) {
    course::attach(server);

    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(server);
}
