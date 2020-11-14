use rusqlite::{ Connection, Error };

pub fn get_connection() -> Result<Connection, Error> {
    Connection::open("myfile.db")
}