use crate::error::{HandledRoute, StatusContext, error};
use crate::state::State;

use afire::prelude::*;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use tracing::trace;

mod auth;
mod test;
mod item;
mod list;

pub fn attach(server: &mut Server<State>) {
    server.route(Method::ANY, "/api/**", |_| {
        Response::new()
            .status(Status::NotFound)
            .text("Route not found")
    });

    auth::attach(server);
    test::attach(server);
    list::attach(server);
    item::attach(server);
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
            "Bad Request: {s} {}",
            serde_json::from_str::<T>(&s).unwrap_err()
        );
        "Bad Request"
    })
}
