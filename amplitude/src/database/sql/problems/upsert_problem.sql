INSERT INTO problems (
    user_id,
    problem_id,
    code,
    completed
    )
VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO
UPDATE
SET
    code = ?3,
    completed = MAX(completed, ?4)