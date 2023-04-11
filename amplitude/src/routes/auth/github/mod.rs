use afire::Server;

use crate::state::State;

mod redirect;
mod complete;

pub fn attach(server: &mut Server<State>) {
    redirect::attach(server);
    complete::attach(server);
}
