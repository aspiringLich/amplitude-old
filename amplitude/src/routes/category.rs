use super::*;

#[derive(Deserialize, Debug)]
struct CategoryReq {
    category: String,
}

/// Returns the configuration / information needed to display a course page
pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "api/category", |state, req| {
        let mut req: CategoryReq = json(req)?;
        req.category = req
            .category
            .strip_prefix('/')
            .unwrap_or(&req.category)
            .to_string();

        let parse_data = state.parse_data();
        let course = parse_data
            .categories
            .get(&req.category)
            .with_context(Status::NotFound, || {
                format!("Category `{}` not found", req.category)
            })?;

        Ok(Response::new().json(course)?)
    })
}
