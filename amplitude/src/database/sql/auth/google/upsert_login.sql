INSERT INTO google_users (
        id,
        google_id,
        name,
        avatar_url,
        access_token,
        created
    )
VALUES (?1, ?2, ?3, ?4, ?5, strftime('%s', 'now')) ON CONFLICT DO
UPDATE
SET name = ?3,
    avatar_url = ?4
RETURNING id;