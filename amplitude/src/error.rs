use std::{
    fmt::{self, Display},
    marker, panic,
    sync::Arc,
};

use afire::{Method, Request, Response, Server, Status};
use tracing::trace;

pub(super) trait HandledRoute<T: Sync + Send> {
    fn handled_route(
        &mut self,
        method: Method,
        path: &'static str,
        handler: impl (Fn(&Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    );

    fn handled_stateful_route(
        &mut self,
        method: Method,
        path: &'static str,
        handler: impl (Fn(Arc<T>, &Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    );
}

impl<T: Send + Sync> HandledRoute<T> for Server<T> {
    fn handled_route(
        &mut self,
        method: Method,
        path: &'static str,
        handler: impl (Fn(&Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    ) {
        self.route(method, path, move |req| {
            let err = handler(req);
            err.unwrap_or_else(|e| {
                trace!("{}: {}", &path, e.body.as_ref().unwrap_or(&"".to_string()));
                Response::new().status(e.status).text(
                    e.body
                        .unwrap_or_else(|| e.status.reason_phrase().to_string()),
                )
            })
        });
    }

    fn handled_stateful_route(
        &mut self,
        method: Method,
        path: &'static str,
        handler: impl (Fn(Arc<T>, &Request) -> Result<Response, StatusError>) + Sync + Send + 'static,
    ) {
        self.stateful_route(method, path, move |state, req| {
            let err = handler(state, req);
            err.unwrap_or_else(|e| {
                trace!("{}: {}", &path, e.body.as_ref().unwrap_or(&"".to_string()));
                Response::new().status(e.status).text(
                    e.body
                        .unwrap_or_else(|| e.status.reason_phrase().to_string()),
                )
            })
        });
    }
}

/// An error that's meant to be converted into some kinda 404 page
pub(super) struct StatusError {
    status: Status,
    body: Option<String>,
}

impl<D: fmt::Display + marker::Sync + marker::Send + 'static> From<D> for StatusError {
    #[track_caller]
    fn from(err: D) -> Self {
        Self {
            status: Status::InternalServerError,
            body: Some(format!("[{}]: {}", std::panic::Location::caller(), err)),
        }
    }
}

pub(super) trait StatusContext<T>
where
    Self: Sized,
{
    /// Bad name but whatever
    #[track_caller]
    fn context(self, status: Status, body: impl Display) -> Result<T, StatusError>;

    #[track_caller]
    fn with_context<F, D>(self, status: Status, body: F) -> Result<T, StatusError>
    where
        D: Display,
        F: FnOnce() -> D;
}

impl<T, E> StatusContext<T> for Result<T, E>
where
    E: Display,
{
    fn context(self, status: Status, body: impl Display) -> Result<T, StatusError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(StatusError {
                status,
                body: Some(format!("{}\n[{}]: {}", body, panic::Location::caller(), e,)),
            }),
        }
    }

    fn with_context<F, D>(self, status: Status, body: F) -> Result<T, StatusError>
    where
        D: Display,
        F: FnOnce() -> D,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(StatusError {
                status,
                body: Some(format!(
                    "{}\n[{}]: {}",
                    Box::new(body()),
                    panic::Location::caller(),
                    e,
                )),
            }),
        }
    }
}

impl<T> StatusContext<T> for Option<T> {
    fn context(self, status: Status, body: impl Display) -> Result<T, StatusError> {
        match self {
            Some(t) => Ok(t),
            None => Err(StatusError {
                status,
                body: Some(format!("{}\n[{}]", body, std::panic::Location::caller())),
            }),
        }
    }

    fn with_context<F, D>(self, status: Status, body: F) -> Result<T, StatusError>
    where
        D: Display,
        F: FnOnce() -> D,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(StatusError {
                status,
                body: Some(format!(
                    "{}\n[{}]",
                    Box::new(body()),
                    std::panic::Location::caller()
                )),
            }),
        }
    }
}

pub(crate) fn error<T>(status: Status, msg: &str) -> Result<T, StatusError> {
    Err(StatusError {
        status,
        body: Some(msg.to_string()),
    })
}
