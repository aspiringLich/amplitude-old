use std::collections::HashMap;

use crate::error::{HandledRoute, StatusContext};
use crate::session::get_session;
use crate::state::State;

use afire::prelude::*;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::trace;

mod auth;
mod category;
mod class;
mod evaluate;
mod exercise;
mod list;
mod problem;

pub fn attach(server: &mut Server<State>) {
    server.route(Method::ANY, "/api/**", |_| {
        Response::new()
            .status(Status::NotFound)
            .text("Route not found")
    });

    auth::attach(server);
    category::attach(server);
    class::attach(server);
    evaluate::attach(server);
    list::attach(server);
    exercise::attach(server);
    problem::attach(server);
}

pub fn json<T>(req: &Request) -> anyhow::Result<T>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    use anyhow::Context;
    let s = Context::context(
        String::from_utf8((*req.body).clone()),
        "Error in request body (Invalid UTF-8)",
    )?;
    Context::with_context(serde_json::from_str(&s), || {
        trace!(
            "Bad Request: `{s}` {}",
            serde_json::from_str::<T>(&s).unwrap_err()
        );
        "Bad Request"
    })
}

pub trait ResponseJson {
    fn json<T: serde::Serialize>(self, data: T) -> Result<Response, serde_json::Error>;
}

impl ResponseJson for Response {
    fn json<T: serde::Serialize>(self, data: T) -> Result<Response, serde_json::Error> {
        Ok(self
            .header("Content-Type", "application/json")
            .text(serde_json::to_string(&data)?))
    }
}

/// Turns a vec of problem ids [`<category>/<problem>`] into a map of
/// `<category> -> [<problem>]`
pub fn mapify_problem_ids(ids: Vec<String>) -> anyhow::Result<ProblemIdMap> {
    let mut map = HashMap::new();
    for id in ids {
        let (category, problem) = id
            .split_once('/')
            .ok_or_else(|| anyhow::anyhow!("Invalid problem id: {}", id))?;

        map.entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(problem.to_string());
    }

    Ok(map)
}

pub type ProblemIdMap = HashMap<String, Vec<String>>;
