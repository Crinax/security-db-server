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
    OrmError(diesel::result::Error),
}

impl<T> From<diesel::result::Error> for DbError<T> {
    fn from(value: diesel::result::Error) -> Self {
        Self::OrmError(value)
    }
}

pub trait DbProvider<Pool, Connection> {
    fn apply<T, E: std::fmt::Debug>(
        &self,
        clojure: impl Fn(&mut Connection) -> Result<T, E>,
    ) -> Result<T, DbError<E>>;
    fn migrate(&self, migraitons: EmbeddedMigrations) -> Result<(), DbError<()>>;
    fn transaction<T, E: std::fmt::Debug>(
        &self,
        clojure: impl Fn(&mut Connection) -> Result<T, E>,
    ) -> Result<T, DbError<E>>;
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
    fn apply<T, E: std::fmt::Debug>(
        &self,
        clojure: impl Fn(&mut PgConnection) -> Result<T, E>,
    ) -> Result<T, DbError<E>> {
        match self.pool.get() {
            Ok(mut connection) => match clojure(&mut connection) {
                Ok(result) => Ok(result),
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(DbError::Execution(err))
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                Err(DbError::Connection)
            }
        }
    }

    fn migrate(&self, migrations: EmbeddedMigrations) -> Result<(), DbError<()>> {
        match self.pool.get() {
            Ok(mut connection) => match connection.run_pending_migrations(migrations) {
                Ok(_) => Ok(()),
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(DbError::Migration)
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                Err(DbError::Connection)
            }
        }
    }

    fn transaction<T, E: std::fmt::Debug>(
        &self,
        clojure: impl Fn(&mut PgConnection) -> Result<T, E>,
    ) -> Result<T, DbError<E>> {
        self.apply(|conn| {
            let clojure_once = |conn: &mut PgConnection| match clojure(conn) {
                Ok(res) => Ok(res),
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(DbError::Execution(err))
                }
            };
            conn.build_transaction().read_write().run(clojure_once)
        })
        .map_err(|err| match err {
            DbError::Execution(err) => err,
            DbError::Connection => DbError::Connection,
            DbError::OrmError(err) => DbError::OrmError(err),
            _ => unreachable!(),
        })
    }
}
