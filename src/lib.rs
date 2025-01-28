mod entities;
mod schema;
mod services;

use pyo3::prelude::*;

use crate::services::interviews::written::WrittenInterview;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
// Create a static singleton pool
pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenv().ok();
    let db_user = env::var("DB_USER").expect("No DB USER ASSIGNED");
    let db_pass = env::var("DB_PASSWORD").expect("No DB PASS ASSIGNED");
    let db_host = env::var("DB_HOST").expect("No DB HOST ASSIGNED");
    let db_port = env::var("DB_PORT").unwrap_or("3306".to_string());
    let db_name = env::var("DB_NAME").expect("No DB NAME ASSIGNED");
    let database_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_pass, db_host,db_port, db_name);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
});

// Helper function to get a reference to the pool
pub fn get_pool() -> &'static DbPool {
    &DB_POOL
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn portal_services(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<WrittenInterview>()?;
    Ok(())
}
