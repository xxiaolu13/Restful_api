use std::sync::Mutex;
use super::models::Course;
#[derive(Debug)]
pub struct AppState {
    pub health_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            health_response: "It's health".into(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![])
        }
    }
}


