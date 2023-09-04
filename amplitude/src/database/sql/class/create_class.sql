-- A class that many users can join through 'user_class'
CREATE TABLE IF NOT EXISTS class (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created INTEGER NOT NULL
)