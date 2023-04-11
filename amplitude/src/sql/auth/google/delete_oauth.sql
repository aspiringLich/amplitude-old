DELETE FROM google_oauth_state
WHERE created < ?;