use std::collections::HashMap;
use crate::state::AppState;
use actix_web::{web,HttpResponse};

pub async fn health_handler(data: web::Data<AppState>) -> HttpResponse {
    let health_response = &data.health_response;
    let mut visit_count = data.visit_count.lock().unwrap();
    let res1 = format!("{}=>{}",health_response,&visit_count);
    *visit_count += 1;
    let mut map = HashMap::new();
    map.insert("message",res1);
    HttpResponse::Ok().json(map)
}