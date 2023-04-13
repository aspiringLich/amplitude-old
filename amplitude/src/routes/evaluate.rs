use std::borrow::Borrow;

use afire::{Content, Method, Response, Server};
use serde::Deserialize;

use crate::{
    runner::{self, Language},
    state::State,
};

#[derive(Deserialize)]
struct EvaluateReq {
    code: String,
    lang: String,
    args: String,
}

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/api/evaluate", |app, req| {
        let body = String::from_utf8_lossy(&req.body);
        let body = serde_json::from_str::<EvaluateReq>(body.borrow()).unwrap();

        let lang = Language::from_str(&body.lang).expect("Invalid language");
        let res = runner::run(app, lang, &body.code, &body.args).unwrap();

        Response::new()
            .text(serde_json::to_string(&res).unwrap())
            .content(Content::JSON)
    });
}
