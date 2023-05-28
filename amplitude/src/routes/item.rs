use super::*;

#[derive(Deserialize, Debug)]
pub struct ItemReq {
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/item", |state, req| {
        let req: ItemReq = json(req)?;
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
