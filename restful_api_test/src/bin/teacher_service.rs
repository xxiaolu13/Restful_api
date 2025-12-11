use actix_web::{web, App, HttpServer,HttpResponse,middleware};
use state::AppState;

#[path ="../state.rs"]
mod state;
#[path="../handlers.rs"]
mod handlers;
#[path="../models.rs"]
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let share_state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim()) // 移除末尾的 /
            .app_data(share_state.clone())
            .route("/", web::get().to(handlers::health_handler)) //router
            .route("/courses", web::post().to(handlers::new_course_handler))
            // curl -X POST localhost:8080/add/course -H "Content-Type: application/json" -d '{"teacher_id":1,"name":"First course"}'
            // curl -X POST localhost:8080/add/course -H "Content-Type: application/json" -d '{"teacher_id":2,"name":"Second course"}'
            .route("/courses", web::get().to(handlers::get_all_courses))
            .service(
                web::scope("/teachers/{teacher_id}/courses")
                .route("", web::get().to(handlers::get_courses_by_teacher_id))
                .route("/{id}", web::get().to(handlers::get_course_by_teacherid_and_id))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
