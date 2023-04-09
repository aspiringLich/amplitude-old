use crate::error::*;
use afire::{extension::ServeStatic, prelude::*};
use amplitude_common::{
    config,
    state::{ArticleRef, State},
};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

mod article;
mod quiz;

#[derive(Deserialize, Debug)]
pub struct ArticleReq {
    pub course: String,
    pub track: String,
    pub article: Option<String>,
}

impl ArticleReq {
    pub fn into_article_ref(self) -> ArticleRef {
        let mut vec = vec![self.course, self.track];
        if let Some(article) = self.article {
            vec.push(article);
        }
        ArticleRef { levels: vec }
    }

    pub fn into_path(self) -> PathBuf {
        let mut path = PathBuf::from(config::RENDERED.clone());
        path.push(self.course);
        path.push(self.track);
        if let Some(article) = self.article {
            path.push(article);
        } else {
            path.push("index");
        }
        path.with_extension("html")
    }
}

pub fn attach(server: &mut Server<State>) {
    article::attach(server);
    quiz::attach(server);

    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(server);
}
