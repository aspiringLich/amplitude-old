use afire::{
    middleware::{MiddleResult, Middleware},
    trace::{Formatter, Level},
    Request,
};
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

pub struct RequestLogger;

impl Middleware for RequestLogger {
    fn pre(&self, req: &mut Request) -> MiddleResult {
        let query = match req.query.is_empty() {
            true => String::new(),
            false => req.query.to_string(),
        };
        event!(
            tracing::Level::TRACE,
            "{} {}{}",
            req.method,
            req.path,
            query
        );
        MiddleResult::Continue
    }
}
