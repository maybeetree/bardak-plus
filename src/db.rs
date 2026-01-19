use sqlite::Connection;
use sqlite::ConnectionThreadSafe;
use sqlite::Error;

//use crate::schema::PostRecord;
//use crate::schema::GetTop;
//use crate::schema::ResponseGetTop;
//use crate::schema::ResponseGetTopInner;

pub fn get_db() -> Result<ConnectionThreadSafe, Error> {
    let connection = Connection::open_thread_safe("scoreboard.sqlite")?;

    init_db(&connection)?;

    Ok(connection)
}

const INIT_QUERY: &str = include_str!("../init.sql");

pub fn init_db(conn: &ConnectionThreadSafe) -> Result<(), Error> {
    conn.execute(INIT_QUERY)
}
