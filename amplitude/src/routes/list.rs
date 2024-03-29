use super::*;

#[derive(Deserialize, Debug)]
struct CategoryReq {
    category: String,
}

/// Returns the list of articles in a course
pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/list", |state, req| {
        let req: CategoryReq = json(req)?;

        let parse_data = state.parse_data();
        let tree = parse_data
            .tree
            .get(&req.category)
            .context(Status::NotFound, "Category not found")?;

        Ok(Response::new()
            .text(serde_json::to_string(tree)?)
            .content(Content::JSON))
    });
    server.handled_stateful_route(Method::GET, "/api/list", |state, _req| {
        Ok(Response::new()
            .text(serde_json::to_string(&state.parse_data().categories)?)
            .header("Access-Control-Allow-Origin", "*")
            .content(Content::JSON))
    });
}
