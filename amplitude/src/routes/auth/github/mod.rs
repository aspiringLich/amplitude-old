use afire::Server;

use crate::state::State;

mod redirect;

pub fn attach(server: &mut Server<State>) {
    redirect::attach(server);
}
