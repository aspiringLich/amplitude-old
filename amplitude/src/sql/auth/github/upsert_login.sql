INSERT INTO github_users (
        id,
        github_id,
        name,
        login,
        avatar_url,
        token,
        created
    )
VALUES (?1, ?2, ?3, ?4, ?5, ?6, strftime('%s', 'now')) ON CONFLICT DO
UPDATE
SET token = ?6,
    name = ?3,
    login = ?4,
    avatar_url = ?5
RETURNING id;