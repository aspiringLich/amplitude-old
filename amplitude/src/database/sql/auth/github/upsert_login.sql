INSERT INTO github_users (
        id,
        github_id,
        login,
        token
    )
VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO
UPDATE
SET login = ?3,
    token = ?4;