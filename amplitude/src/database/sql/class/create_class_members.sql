-- Puts a user in a class
CREATE TABLE IF NOT EXISTS class_members (
  class_id INTEGER NOT NULL,
  user_id TEXT NOT NULL,
  date_added INTEGER NOT NULL
)