use actix_web::{web,HttpResponse};
use crate::models::course::{CreateCourse,UpdateCourse};
use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::state::AppState;

// c
pub async fn post_new_course_handler(new_course:web::Json<CreateCourse>,app_state:web::Data<AppState>)-> Result<HttpResponse, MyError>{
    post_new_course_db(&app_state.db_pool,new_course.try_into()?)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}


// r
pub async fn get_all_courses(app_state:web::Data<AppState>)-> Result<HttpResponse, MyError>{
    get_all_courses_db(&app_state.db_pool)
    .await
    .map(|all_courses|HttpResponse::Ok().json(all_courses))
}


// r
pub async fn get_courses_by_teacher_id(app_state:web::Data<AppState>,path: web::Path<usize>) -> Result<HttpResponse, MyError>{
    let teacher_id  =  path.into_inner() as i32;
    get_courses_by_teacher_id_db(&app_state.db_pool,teacher_id)
    .await
    .map(|courses_list|HttpResponse::Ok().json(courses_list))
}


// r
pub async fn get_course_by_id(app_state:web::Data<AppState>,path: web::Path<i32>)->Result<HttpResponse,MyError>{
    let id = path.into_inner();
    get_courses_by_id_db(&app_state.db_pool,id)
    .await
    .map(|course_result|HttpResponse::Ok().json(course_result))
}


// d
pub async fn delete_course_by_teacherid_id(app_state:web::Data<AppState>,path: web::Path<(i32,i32)>)->Result<HttpResponse,MyError>{
    let (id,teacher_id) = path.into_inner();
    delete_course_by_teacherid_id_db(&app_state.db_pool,teacher_id,id).await
    .map(|course_result|HttpResponse::Ok().json(course_result))
}


// u
pub async fn update_course_by_teacherid_id(app_state:web::Data<AppState>,path: web::Path<(i32,i32)>,updatecourse:web::Json<UpdateCourse>)->Result<HttpResponse,MyError>{
    let (id,teacher_id) = path.into_inner();
    update_course_by_teacherid_id_db(&app_state.db_pool,teacher_id,id,updatecourse.try_into()?).await
    .map(|course_result|HttpResponse::Ok().json(course_result))
}








#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[actix_rt::test]
    async fn post_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_state = web::Data::new(AppState::new(db_pool));
        let course = web::Json(CreateCourse{
            teacher_id: 345,
            name: "test345".into(),
            description: Some("345".into()),
            format: None,
            structure: None,
            duration: None,
            language: Some("345".into()),
            level: None,
            price: None
        });
        let res = post_new_course_handler(course, share_state).await.unwrap();
        assert_eq!(res.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_test1(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_state = web::Data::new(AppState::new(db_pool));
        let teacher_id = web::Path::from(1);
        let res = get_courses_by_teacher_id(share_state, teacher_id).await.unwrap();
        assert_eq!(res.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_test2(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_state = web::Data::new(AppState::new(db_pool));
        let id = web::Path::from(500000);
        let res = get_course_by_id(share_state, id).await;
        match res{
            Ok(_) => println!("get course test2 make a bug"),
            Err(err) => assert_eq!(err.status_code(),StatusCode::NOT_FOUND)
        }     
    }

    #[actix_rt::test]
    async fn update_course_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_state = web::Data::new(AppState::new(db_pool));
        let update_course = web::Json(UpdateCourse{
            name: Some("update_test".into()),
            description: None,
            structure:None,
            duration:None,
            format:None,
            price: None,
            language: None,
            level: None
        });
        let path = web::Path::from((11,1111));
        let res = update_course_by_teacherid_id(share_state, path, update_course).await.unwrap();
        assert_eq!(res.status(),StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_course_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NotFound");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_state = web::Data::new(AppState::new(db_pool));
        let path = web::Path::from((16,1111));
        let res = delete_course_by_teacherid_id(share_state, path).await;
        match res{
            Ok(_) => println!("get course test delete make a bug"),
            Err(err) => assert_eq!(err.status_code(),StatusCode::NOT_FOUND)
        }
    }


}