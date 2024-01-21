use rusqlite::Connection;

pub(crate) fn get_database_connection() -> Connection {
    Connection::open("db.db").expect("Err opening DB")
}