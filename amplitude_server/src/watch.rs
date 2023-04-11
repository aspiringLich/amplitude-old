use std::{sync::{self, Arc}, thread, time};

use amplitude_common::config;
use amplitude_markdown::{state::ParseState, parse::parse_dir};
use amplitude_state::State;
use notify::{RecommendedWatcher, Config, Watcher};
use parking_lot::RwLock;
use tracing::{info, error};

/// This function will watch the input directory and write to the output
/// directory when detecting file changes using the `notify` crate.
///
/// See [`parse_dir`] for more description on how this function behaves
pub fn parse_dir_watch(state: Arc<State>) -> notify::Result<()> {
    let (tx, rx) = sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(config::INPUT.as_path(), notify::RecursiveMode::Recursive)?;

    info!("Watching for changes in '{}'", config::INPUT);

    while let Ok(mut event) = rx.recv() {
        use notify::EventKind::*;

        // wait 50ms to avoid duplicate events
        thread::sleep(time::Duration::from_millis(50));

        // drain the channel
        while let Ok(e) = rx.try_recv() {
            match e {
                Ok(e) if matches!(e.kind, Create(_) | Modify(_) | Remove(_)) => event = Ok(e),
                Err(e) => error!("Error watching directory: {:?}", e),
                _ => (),
            }
        }

        match event {
            Ok(event) if matches!(event.kind, Create(_) | Modify(_) | Remove(_)) => {
                info!("Change detected, reparsing...");
                match parse_dir(&config::INPUT, &config::RENDERED) {
                    Err(e) => error!("Error parsing directory: '{:?}'", e),
                    Ok(s) => *state.parse_state.write() = s,
                }
            }
            Err(e) => error!("Error watching directory: {:?}", e),
            _ => (),
        }
    }

    Ok(())
}
