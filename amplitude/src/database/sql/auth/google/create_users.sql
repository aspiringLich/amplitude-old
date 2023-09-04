-- Main table for holding users who have logged in with Google
CREATE TABLE IF NOT EXISTS google_users (
   id TEXT NOT NULL UNIQUE,        -- Amplitude user id
   google_id TEXT NOT NULL UNIQUE, -- Google ID
   access_token TEXT               -- Google user access token
)