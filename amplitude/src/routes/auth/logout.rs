//! Delete the user's current session (on db and cookie) and redirect to the home page.

use afire::{Method, Response, Server, SetCookie, Status};

use crate::{error::HandledRoute, misc::OkResponse, session::get_session, state::State};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/logout", move |app, req| {
        if let Ok(i) = get_session(&app, req) {
            app.db.session().delete_session(&i.token)?;
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
