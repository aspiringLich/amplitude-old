-- Session cookies
CREATE TABLE IF NOT EXISTS sessions (
   created INTEGER NOT NULL,  -- Time created in epoch
   user_id TEXT NOT NULL,     -- Amplitude user id
   session_id TEXT NOT NULL,  -- Session id
   platform INTEGER NOT NULL, -- Platform id (google / github)
   user_agent TEXT            -- User agent of device that created session
)