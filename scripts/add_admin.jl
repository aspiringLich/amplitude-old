using SQLite
using DataFrames

DB_FILE = "data.db"
USER_ID = ""

db = SQLite.DB(DB_FILE)

user = SQLite.DBInterface.execute(db, "SELECT COUNT(*) FROM users WHERE id = ?", [USER_ID]) |> DataFrame
if user[1, 1] == 0
    println("User does not exist")
    exit(1)
end

SQLite.DBInterface.execute(db, "UPDATE users SET admin = 1 WHERE id = ?", [USER_ID])
println("User is now admin")