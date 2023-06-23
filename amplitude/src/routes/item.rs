use super::*;

#[derive(Deserialize, Debug)]
pub struct ItemReq {
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/item", |state, req| {
        let mut req: ItemReq = json(req)?;
        req.id = req.id.strip_prefix('/').unwrap_or(&req.id).to_string();
        req.id = req.id.strip_suffix('/').unwrap_or(&req.id).to_string();

        let parse_data = state.parse_data();
        let item = parse_data
            .items
            .get(&req.id)
            .with_context(Status::NotFound, || format!("Item `{}` not found", req.id))?;

        let mut buffer = Vec::new();
        let mut s = serde_json::Serializer::new(&mut buffer);

        item.serialize_for_route(&mut s).context(
            Status::InternalServerError,
            "While serializing Item to JSON",
        )?;

        Ok(Response::new()
            .bytes(buffer.as_slice())
            .content(Content::JSON))
    });
}
