CREATE TABLE IF NOT EXISTS users (
   id TEXT NOT NULL UNIQUE,         -- Amplitude user id
   name TEXT NOT NULL,              -- Name
   avatar_url TEXT NOT NULL,        -- Image 
   created INTEGER NOT NULL,        -- Epoch created
   admin INTEGER NOT NULL DEFAULT 0 -- Is admin
)
