use afire::Server;
use amplitude_state::State;
use tracing::{error, info};

mod github;
mod google;

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
        return;
    }

    google::attach(server);
    github::attach(server);
}
