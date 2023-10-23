use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};

pub mod orm;

pub trait DbUrlProvider {
    fn db_url(&self) -> &str;
}

pub enum DbError<T> {
    InstanceError,
    ConnectionError,
    ExecutionError(T),
    BlockingError
}

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    pool: PgPool
}

impl Db {
    pub fn new(host: &str) -> Result<Self, DbError<()>> {
        let manager = ConnectionManager::new(host);
        if let Ok(pool) = Pool::new(manager) {
            return Ok(Self { pool });
        }

        Err(DbError::InstanceError)
    }

    pub fn apply<T, E>(&self, clojure: impl Fn(&mut PgConnection) -> Result<T, E>) -> Result<T, DbError<E>> {
        match self.pool.get() {
            Ok(mut connection) => match clojure(&mut connection) {
                Ok(result) => Ok(result),
                Err(err) => Err(DbError::ExecutionError(err))
            }
            Err(_) => Err(DbError::ConnectionError)
        }
    }
}
