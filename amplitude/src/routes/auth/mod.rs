use afire::Server;
use tracing::{error, info};

use crate::state::State;

mod github;
mod google;
mod logout;
mod session;
mod supported;

// dw breon i got this

/*
Reference Impls:
Github: https://github.com/Basicprogrammer10/amplify/tree/master/src/auth
Google: https://github.com/Basicprogrammer10/coding-hat/tree/master/src/auth
*/

pub fn attach(server: &mut Server<State>) {
    let github = server.app().config.github_oauth.is_some();
    let google = server.app().config.google_oauth.is_some();

    if github {
        info!("Initiating Github oauth");
    }

    if google {
        info!("Initiating Google oauth");
    }

    if !github && !google {
        error!("No auth providers configured");
    }

    google::attach(server);
    github::attach(server);
    logout::attach(server);
    session::attach(server);
    supported::attach(server);
}
