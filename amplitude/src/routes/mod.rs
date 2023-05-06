use crate::{
    error::{HandledRoute, StatusContext},
    state::State,
};
use afire::{extension::ServeStatic, prelude::*};
use amplitude_common::path;
use derive_more::{Deref, DerefMut};
use serde::{de, Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Component, Path, PathBuf},
};

mod article;
mod article_list;
mod auth;
mod course;
mod evaluate;
mod quiz;

pub fn attach(server: &mut Server<State>) {
    article::attach(server);
    auth::attach(server);
    evaluate::attach(server);
    quiz::attach(server);
    course::attach(server);
    article_list::attach(server);

    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(server);
}
