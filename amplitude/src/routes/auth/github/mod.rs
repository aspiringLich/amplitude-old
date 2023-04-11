use afire::Server;

use crate::state::State;

mod complete;
mod redirect;

pub fn attach(server: &mut Server<State>) {
    if server.app().config.github_oauth.is_none() {
        return;
    }

    redirect::attach(server);
    complete::attach(server);
}
