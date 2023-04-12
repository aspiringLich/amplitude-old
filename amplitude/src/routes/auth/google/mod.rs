use afire::Server;

use crate::state::State;

mod complete;
mod redirect;

pub fn attach(server: &mut Server<State>) {
    if server.app().config.google_oauth.is_none() {
        return;
    }

    complete::attach(server);
    redirect::attach(server);
}
