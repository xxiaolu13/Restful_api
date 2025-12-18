use actix_web::{web, App, HttpServer,middleware,HttpResponse, HttpRequest};
use state::AppState;
use sqlx::{postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;
use crate::handler::course::*;
use crate::handler::general::*;
use crate::handler::teacher::*;
use tracing::{info,warn,debug,error};
use tracing_subscriber;
//DATABASE_URL=postgresql://postgres:1@192.168.1.7:5432/postgres
#[path ="../state.rs"]
mod state;
#[path="../handler/mod.rs"]
mod handler;
#[path="../models/mod.rs"]
mod models;
#[path="../dbaccess/mod.rs"]
mod dbaccess;
#[path ="../errors.rs"]
mod errors;


async fn not_found_handler(req: HttpRequest) -> HttpResponse {
    warn!(
        method = %req.method(),
        path = %req.path(),
        "Route not found"
    );
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Route not found",
        "path": req.path(),
        "method": req.method().as_str()
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    tracing_subscriber::fmt::init();

    dotenv().ok();
    info!("Environment variables loaded");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
    let db_pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {:?}", e);
            e
        })
        .unwrap();
    let share_state = web::Data::new(AppState::new(db_pool));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim()) // 移除末尾的 /
            .app_data(share_state.clone())
            .route("/", web::get().to(health_handler)) //router
            // curl -X POST localhost:8080/courses -H "Content-Type: application/json" -d '{"teacher_id":1,"name":"First course"}'
            // curl -X POST localhost:8080/courses -H "Content-Type: application/json" -d '{"teacher_id":2,"name":"Second course"}'
            .service(
                web::scope("/courses")
                .route("", web::get().to(get_all_courses))
                .route("", web::post().to(post_new_course_handler))
                .route("/teacher/{teacher_id}", web::get().to(get_courses_by_teacher_id))
                .route("/id/{id}", web::get().to(get_course_by_id))
                .route("/id/{id}/teacher/{teacher_id}/delete",web::delete().to(delete_course_by_teacherid_id))
                .route("/id/{id}/teacher/{teacher_id}/update",web::put().to(update_course_by_teacherid_id))
            )
            .service(
                web::scope("/teachers")
                .route("", web::get().to(get_all_teachers))
                .route("", web::post().to(post_new_teacher))
                .route("/{teacher_id}" ,web::delete().to(delete_teacher_by_id))
                .route("/{teacher_id}",web::put().to(update_teacher_by_id))
                .route("/{teacher_id}", web::get().to(get_teacher_by_id))
            )
            .default_service(web::route().to(not_found_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
