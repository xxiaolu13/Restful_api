use std::sync::Mutex;
// use super::models::Course;
use sqlx::PgPool;

#[derive(Debug)]
pub struct AppState {
    pub health_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db_pool : PgPool,
}
impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            health_response: "It's health".into(),
            visit_count: Mutex::new(0),
            db_pool,
        }
    }
}


