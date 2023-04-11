-- user id, session id
INSERT INTO sessions VALUES (strftime('%s', 'now'), ?, ?);