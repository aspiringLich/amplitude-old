use std::borrow::Borrow;

use amplitude_common::lang::Language;

use super::*;

use crate::{error::HandledRoute, runner, state::State};

#[derive(Deserialize)]
struct EvaluateReq {
    code: String,
    lang: String,
    args: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/evaluate", |app, req| {
        let body = String::from_utf8_lossy(&req.body);
        let body = serde_json::from_str::<EvaluateReq>(body.borrow())
            .context(Status::BadRequest, "Invalid request")?;
        let lang = Language::from_str(&body.lang)
            .context(Status::InternalServerError, "Invalid language")?;
        let res = runner::run(app, lang, &body.code, &body.args)
            .context(Status::InternalServerError, "Failed to run code")?;

        Ok(Response::new()
            .text(serde_json::to_string(&res)?)
            .content(Content::JSON))
    });
}
