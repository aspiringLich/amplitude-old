-- Table to hold oauth states (used during login)
CREATE TABLE IF NOT EXISTS github_oauth_state (
   state TEXT NOT NULL UNIQUE, -- OAuth state
   redirect TEXT,              -- Redirect URL
   created INTEGER NOT NULL    -- Epoch created
)