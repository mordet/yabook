use postgres::{Connection};

pub fn init_db(conn: &Connection) {
    let commands  = vec![
        "CREATE SCHEMA IF NOT EXISTS db;",
        "CREATE TABLE IF NOT EXISTS db.table(
           name varchar(50) PRIMARY KEY,
           location varchar(50)
         );",
        "CREATE TABLE IF NOT EXISTS db.user(
           name varchar(50) PRIMARY KEY
         );",
        "CREATE TABLE IF NOT EXISTS db.book(
           id SERIAL PRIMARY KEY,
           start timestamp NOT NULL,
           duration interval NOT NULL,
           table_name varchar(50) NOT NULL REFERENCES db.table (name),
           owner varchar(50) NOT NULL REFERENCES db.user (name),
           public boolean NOT NULL
        );",
        "DO $$
         BEGIN
           IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
             CREATE TYPE db.status AS ENUM (
               'sent',
               'accepted',
               'declined'
             );
           END IF;
         END$$;",
        "CREATE TABLE IF NOT EXISTS db.invite(
           book_id int NOT NULL REFERENCES db.book (id),
           user_name varchar(50) NOT NULL REFERENCES db.user (name),
           status db.status NOT NULL
         );"
    ];

    for command in commands.iter() {
        conn.execute(command, &[]).unwrap();
    }
}
