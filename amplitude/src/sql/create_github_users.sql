CREATE TABLE IF NOT EXISTS github_users (
   id TEXT NOT NULL UNIQUE,   -- Github ID
   name TEXT NOT NULL,        -- Name (from github)
   login TEXT NOT NULL,       -- Login (from github)
   avatar_url TEXT NOT NULL,  -- Avatar image link
   token TEXT,                -- Github user access token
   created INTEGER NOT NULL   -- Epoch created
)