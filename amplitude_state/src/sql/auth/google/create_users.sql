-- Main table for holding users who have logged in with Google
CREATE TABLE IF NOT EXISTS google_users (
   id TEXT NOT NULL UNIQUE,  -- Google ID
   name TEXT NOT NULL,       -- Name (from google)
   avatar_url TEXT NOT NULL, -- Image (from google)
   access_token TEXT,        -- Google user access token
   created INTEGER NOT NULL  -- Epoch created
)