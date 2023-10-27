pub mod models;
pub mod orm;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    pool: PgPool,
}

pub trait DbUrlProvider {
    fn db_url(&self) -> &str;
}

pub trait DbErrorProvider {
    fn as_text(&self) -> &str;
}

#[derive(Debug)]
pub enum DbError<T> {
    Instance,
    Connection,
    Execution(T),
    Migration,
}

pub trait DbProvider<Pool, Connection> {
    fn apply<T, E>(
        &self,
        clojure: impl Fn(&mut Connection) -> Result<T, E>,
    ) -> Result<T, DbError<E>>;
    fn migrate(&self, migraitons: EmbeddedMigrations) -> Result<(), DbError<()>>;
}

impl Db {
    pub fn new(host: &str) -> Result<Self, DbError<()>> {
        let manager = ConnectionManager::new(host);
        if let Ok(pool) = Pool::new(manager) {
            return Ok(Self { pool });
        }

        Err(DbError::Instance)
    }
}

impl DbProvider<PgPool, PgConnection> for Db {
    fn apply<T, E>(
        &self,
        clojure: impl Fn(&mut PgConnection) -> Result<T, E>,
    ) -> Result<T, DbError<E>> {
        match self.pool.get() {
            Ok(mut connection) => match clojure(&mut connection) {
                Ok(result) => Ok(result),
                Err(err) => Err(DbError::Execution(err)),
            },
            Err(_) => Err(DbError::Connection),
        }
    }

    fn migrate(&self, migrations: EmbeddedMigrations) -> Result<(), DbError<()>> {
        match self.pool.get() {
            Ok(mut connection) => match connection.run_pending_migrations(migrations) {
                Ok(_) => Ok(()),
                Err(_) => Err(DbError::Migration),
            },
            Err(_) => Err(DbError::Connection),
        }
    }
}
