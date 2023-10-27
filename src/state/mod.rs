use std::sync::Arc;

use super::db::PgPool;
use diesel::PgConnection;

use crate::db::DbProvider;

pub struct AppState<T> {
    db: Arc<T>,
}

impl<T: DbProvider<PgPool, PgConnection>> AppState<T> {
    pub fn new(db: Arc<T>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &T {
        &self.db
    }
}
