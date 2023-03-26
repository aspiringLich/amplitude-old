use afire::prelude::*;
use amplitude_common::config;
use std::fmt::Display;

mod course;

pub fn attach(server: &mut Server) {
    course::attach(server);
}

/// An error that's meant to be converted into some kinda 404 page
pub(super) struct StatusError {
    status: Status,
    body: Option<String>,
}

impl<E: std::error::Error + std::marker::Sync + std::marker::Send + 'static> From<E>
    for StatusError
{
    #[track_caller]
    fn from(err: E) -> Self {
        Self {
            status: Status::InternalServerError,
            body: Some(format!("[{}]: {}", std::panic::Location::caller(), err)),
        }
    }
}

pub(super) trait StatusContext<T> {
    /// Bad name but whatever
    fn status(self, status: Status, body: impl Display) -> Result<T, StatusError>;
}

impl<T, E> StatusContext<T> for Result<T, E>
where
    E: std::error::Error,
{
    #[track_caller]
    fn status(self, status: Status, body: impl Display) -> Result<T, StatusError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(StatusError {
                status,
                body: Some(format!(
                    "{}\n[{}]: {}",
                    e,
                    std::panic::Location::caller(),
                    body
                )),
            }),
        }
    }
}

impl<T> StatusContext<T> for Option<T> {
    #[track_caller]
    fn status(self, status: Status, body: impl Display) -> Result<T, StatusError> {
        match self {
            Some(t) => Ok(t),
            None => Err(StatusError {
                status,
                body: Some(format!("[{}]: {}", std::panic::Location::caller(), body)),
            }),
        }
    }
}

pub(super) trait HandledRoute {
    fn handled_route(
        &mut self,
        method: Method,
        path: &str,
        handler: impl (Fn(&Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    );
}

impl HandledRoute for Server {
    fn handled_route(
        &mut self,
        method: Method,
        path: &str,
        handler: impl (Fn(&Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    ) {
        self.route(method, path, move |req| {
            let err = handler(req);
            err.unwrap_or_else(|e| {
                Response::new().status(e.status).text(
                    &e.body
                        .unwrap_or_else(|| e.status.reason_phrase().to_string()),
                )
            })
        });
    }
}
