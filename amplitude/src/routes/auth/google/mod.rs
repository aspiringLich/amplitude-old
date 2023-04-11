use afire::Server;

use crate::state::State;

mod redirect;

pub fn attach(server: &mut Server<State>) {
    if server.app().config.google_oauth.is_none() {
        return;
    }

    redirect::attach(server);
}
