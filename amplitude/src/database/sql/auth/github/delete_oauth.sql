DELETE FROM github_oauth_state
WHERE created < ?;