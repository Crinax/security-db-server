use diesel_migrations::EmbeddedMigrations;

pub trait DbUrlProvider {
    fn db_url(&self) -> &str;
}

pub trait DbErrorProvider {
    fn as_text(&self) -> &str;
}

#[derive(Debug)]
pub enum DbError<T> {
    InstanceError,
    ConnectionError,
    ExecutionError(T),
    MigrationError,
}

pub trait DbProvider<Pool, Connection> {
    fn apply<T, E>(&self, clojure: impl Fn(&mut Connection) -> Result<T, E>) -> Result<T, DbError<E>>;
    fn migrate(&self, migraitons: EmbeddedMigrations) -> Result<(), DbError<()>>;
}
