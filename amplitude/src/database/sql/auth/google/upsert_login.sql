INSERT INTO google_users (
        id,
        google_id,
        access_token
    )
VALUES (?1, ?2, ?3) ON CONFLICT DO
UPDATE
SET google_id = ?2,
    access_token = ?3;
SELECT id
FROM google_users
WHERE google_id = ?2;