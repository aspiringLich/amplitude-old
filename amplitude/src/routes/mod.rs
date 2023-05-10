use crate::error::{HandledRoute, StatusContext};
use crate::state::State;

use afire::{extension::ServeStatic, prelude::*};

use serde::{Deserialize, Serialize};
use std::fs::File;

mod article;
mod article_list;
mod auth;
mod evaluate;
mod quiz;

pub fn attach(server: &mut Server<State>) {
    server.route(Method::ANY, "/api/**", |_| {
        Response::new()
            .status(Status::NotFound)
            .text("Route not found")
    });
    
    article::attach(server);
    auth::attach(server);
    evaluate::attach(server);
    quiz::attach(server);
    article_list::attach(server);

    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(server);
}
