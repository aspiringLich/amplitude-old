-- Main table for holding users who have logged in with Github
CREATE TABLE IF NOT EXISTS github_users (
   id TEXT NOT NULL UNIQUE,           -- Amplitude user id
   github_id INTEGER NOT NULL UNIQUE, -- Github ID
   login TEXT NOT NULL,               -- Login (from github)
   token TEXT                         -- Github user access token
)