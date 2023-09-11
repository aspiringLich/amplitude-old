use amplitude_markdown::items::exercise::transform;

use super::*;

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/exercise", |state, req| {
        #[derive(Deserialize, Debug)]
        struct ExerciseReq {
            id: String,
        }

        let mut req: ExerciseReq = json(req)?;
        req.id = req.id.strip_prefix('/').unwrap_or(&req.id).to_string();

        let parse_data = state.parse_data();
        let e = parse_data
            .exercises
            .get(&req.id)
            .with_context(Status::NotFound, || {
                format!("Exercise `{}` not found", req.id)
            })?;

        let mut clone = e.clone();
        transform(&mut clone);

        Ok(Response::new().json(clone)?)
    });
    server.handled_stateful_route(Method::POST, "/api/exercise/category", |state, req| {
        #[derive(Deserialize, Debug)]
        struct ExerciseReq {
            category: String,
        }

        let req: ExerciseReq = json(req)?;

        let parse_data = state.parse_data();
        let category = parse_data
            .categories
            .get(&req.category)
            .with_context(Status::NotFound, || {
                format!("Category `{}` not found", req.category)
            })?;

        let exercises = category
            .exercises
            .iter()
            .map(|e| (e, &parse_data.exercises.get(e).unwrap().config))
            .collect::<HashMap<_, _>>();

        Ok(Response::new().json(exercises)?)
    })
}
