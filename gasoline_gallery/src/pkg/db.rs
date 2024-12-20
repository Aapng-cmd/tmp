use diesel::{pg::PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use std::env;

use crate::error::AppError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn connection() -> Result<DbConnection, AppError> {
    POOL.get()
        .map_err(|e| AppError::new(format!("Failed getting db connection: {}", e)))
}