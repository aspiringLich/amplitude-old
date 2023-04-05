use crate::error::*;
use afire::{extension::ServeStatic, prelude::*};
use amplitude_common::{config, state::State};
use serde::Deserialize;
use std::fs::File;

mod course;
mod quiz;

pub fn attach(server: &mut Server<State>) {
    course::attach(server);
    quiz::attach(server);

    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(server);
}
