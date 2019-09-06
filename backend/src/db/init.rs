extern crate postgres;

use postgres::{Connection};

struct Table {
    name: String,
    location: String,
}

struct Users {
    name: String,
}

struct Book {
    id: i32,
    start: usize,
    duration: i32,
    table_name: String,
    owner: String,
    public: bool,
}

pub fn init_db(conn:Connection) {
    conn.execute(
        concat!(
        "CREATE SCHEMA IF NOT EXISTS db;

        CREATE TABLE IF NOT EXISTS db.table(
          name varchar(50) PRIMARY KEY,
          location varchar(50)
        );

        CREATE TABLE IF NOT EXISTS db.user(
          name varchar(50) PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS db.book(
          id SERIAL PRIMARY KEY,
          start timestamp NOT NULL,
          duration interval NOT NULL,
          table_name varchar(50) NOT NULL REFERENCES db.table (name),
          owner varchar(50) NOT NULL REFERENCES db.user (name),
          public boolean NOT NULL
        );

        DO $$
        BEGIN
          IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'db.status') THEN
            CREATE TYPE db.status AS ENUM (
              'sent',
              'accepted',
              'declined'
            );
          END IF;
        END$$;

        CREATE TABLE IF NOT EXISTS invite(
          book_id int NOT NULL REFERENCES db.book (id),
          user_name varchar(50) NOT NULL REFERENCES db.user (name),
          status db.status NOT NULL
        );
        "),
               &[])
      .unwrap();
}