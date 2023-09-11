use super::*;

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/problem/progress/save", |app, req| {
        let session = get_session(&app, req)?;

        #[derive(Debug, Deserialize)]
        struct ProgressSaveRequest {
            problem_id: String,
            code: String,
        }

        let data = json::<ProgressSaveRequest>(req)?;
        app.db
            .misc()
            .save_problem_progress(&session.id, &data.problem_id, &data.code)?;

        Ok(Response::new())
    });

    server.handled_stateful_route(Method::GET, "/api/problem/progress/load", |app, req| {
        let session = get_session(&app, req)?;

        #[derive(Debug, Deserialize)]
        struct ProgressLoadRequest {
            problem_id: String,
        }
        let ProgressLoadRequest { problem_id } = json(req)?;
        app.db
            .misc()
            .load_problem_progress(&session.id, &problem_id)?;

        todo!()
    });

    server.handled_stateful_route(Method::GET, "/api/problem/completion", |app, req| {
        let session = get_session(&app, req);
        #[derive(Serialize, Default)]
        struct ProblemCompletion {
            completed: ProblemIdMap,
            incomplete: ProblemIdMap,
        }

        if let Ok(session) = session {
            let completed_ids = app.db.misc().get_completed_problems(&session.id)?;
            let incomplete_ids = app.db.misc().get_incomplete_problems(&session.id)?;

            let status = ProblemCompletion {
                completed: mapify_problem_ids(completed_ids)?,
                incomplete: mapify_problem_ids(incomplete_ids)?,
            };

            Ok(Response::new().json(status)?)
        } else {
            Ok(Response::new().json(ProblemCompletion::default())?)
        }
    });
}
