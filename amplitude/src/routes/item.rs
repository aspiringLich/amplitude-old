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
        let state = &state.parse_data;

        let item = state
            .items
            .get(&req.id)
            .with_context(Status::NotFound, || format!("Item `{}` not found", req.id))?;

        Ok(Response::new()
            .text(serde_json::to_string(item)?)
            .content(Content::JSON))
    });
}
