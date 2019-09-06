pub mod book;
pub mod init;
pub mod table;
pub mod user;

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>;

pub fn get_pool() -> Pool {
    crate::CONNECTION.clone()
}
