use amplitude_markdown::items::exercise::transform;
use serde::Serialize;

use super::*;

#[derive(Deserialize, Debug)]
pub struct ItemReq {
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/exercise", |state, req| {
        let mut req: ItemReq = json(req)?;
        req.id = req.id.strip_prefix('/').unwrap_or(&req.id).to_string();

        let parse_data = state.parse_data();
        let e = parse_data
            .exercises
            .get(&req.id)
            .with_context(Status::NotFound, || {
                format!("Exercise `{}` not found", req.id)
            })?;

        let mut buffer = Vec::new();
        let mut s = serde_json::Serializer::new(&mut buffer);

        let mut clone = e.clone();
        transform(&mut clone);
        clone.serialize(&mut s).context(
            Status::InternalServerError,
            "While serializing Item to JSON",
        )?;

        Ok(Response::new()
            .bytes(buffer.as_slice())
            .content(Content::JSON))
    });
}
