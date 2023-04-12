use afire::{Method, Response, Server, SetCookie, Status};

use crate::{
    database::Database, error::HandledRoute, misc::OkResponse, session::get_session, state::State,
};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/logout", move |app, req| {
        if let Ok(i) = get_session(app.clone(), req) {
            app.db().delete_session(&i.token)?;
        }

        // Remove Session Cookie
        Response::new()
            .status(Status::TemporaryRedirect)
            .header("Cache-Control", "no-store")
            .header("Location", "/")
            .cookie(SetCookie::new("session", "LOGOUT").path("/").max_age(0))
            .ok()
    });
}
