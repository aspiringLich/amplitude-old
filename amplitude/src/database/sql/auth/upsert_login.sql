INSERT INTO users (
        id,
        name,
        avatar_url,
        created,
        admin
    )
VALUES  (?1, ?2, ?3, strftime('%s', 'now'), 0) ON CONFLICT DO
UPDATE
SET
    name = ?2,
    avatar_url = ?3;