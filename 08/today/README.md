# Today 08

After this week's additions, the SQLiteProvider event provider
will be in place. For testing events originated from SQLite, 
place the database file in the configuration directory, then add
its details to the `today.toml` file in the configuration directory:

    [[providers]]
    name = "events-db"
    kind = "sqlite"
    resource = "events.db"

(Replace the information with your own if it is different.)
