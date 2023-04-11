use afire::Server;
use amplitude_state::State;

mod redirect;

pub fn attach(server: &mut Server<State>) {
    redirect::attach(server);
}
