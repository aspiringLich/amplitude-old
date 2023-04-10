use afire::trace::{Formatter, Level};
use tracing::event;

pub struct AfireLogger;

impl Formatter for AfireLogger {
    fn format(&self, level: Level, _color: bool, msg: String) {
        match level {
            Level::Off => {}
            Level::Error => event!(target: "afire::logger", tracing::Level::ERROR, "{}", msg),
            Level::Debug => event!(target: "afire::logger", tracing::Level::DEBUG, "{}", msg),
            Level::Trace => event!(target: "afire::logger", tracing::Level::INFO, "{}", msg),
        }
    }
}
