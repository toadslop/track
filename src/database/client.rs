use sqlx::{Pool, Postgres};
use std::ops::{Deref, DerefMut};

/// A wrapper struct around a connection pool for a Postgres database.
#[derive(Debug, Clone)]
pub struct Database(Pool<Postgres>);

impl Database {
    pub fn inner(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl From<Pool<Postgres>> for Database {
    fn from(value: Pool<Postgres>) -> Self {
        Self(value)
    }
}

impl Deref for Database {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
