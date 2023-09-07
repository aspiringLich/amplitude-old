CREATE TABLE IF NOT EXISTS problems (
    user_id TEXT NOT NULL,
    problem_id TEXT NOT NULL,
    code INTEGER NOT NULL,
    completed INTEGER NOT NULL,
    UNIQUE(user_id, problem_id)
)