use actix_web::{web,HttpResponse};
use crate::models::teacher::{CreateTeacher,UpdateTeacher};
use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::state::AppState;
use tracing::{info, error};

pub async fn get_all_teachers(data:web::Data<AppState>) -> Result<HttpResponse,MyError>{
    let teachers = get_all_teachers_db(&data.db_pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch teachers: {:?}", e);
        e
    })?;
    info!("Found {} teachers", teachers.len());
    Ok(HttpResponse::Ok().json(teachers))

}


pub async fn get_teacher_by_id(data:web::Data<AppState>,params:web::Path<i32>) -> Result<HttpResponse,MyError>{
    let id = params.into_inner();
    let teachers = get_teacher_by_id_db(&data.db_pool, id).await.map_err(
        |e|{
            error!("Failed get teacher by id {:?}",e);
            e
        }
    )?;
    info!("Found teacher by id:{}", teachers.id);
    Ok(HttpResponse::Ok().json(teachers))
}


pub async fn post_new_teacher(data:web::Data<AppState>,teacher:web::Json<CreateTeacher>) -> Result<HttpResponse,MyError>{
    let teachers = post_new_teacher_db(&data.db_pool, teacher.try_into()?).await.map_err(|e|{
        error!("Failed to create a new teacher {:?}",e);
        e
    })?;
    info!("success to create a new teacher, id:{}",teachers.id);
    Ok(HttpResponse::Ok().json(teachers))
}


pub async fn delete_teacher_by_id(data:web::Data<AppState>,params:web::Path<i32>) -> Result<HttpResponse,MyError>{
    let id = params.into_inner();
    let teachers = delete_teacher_by_id_db(&data.db_pool, id).await.map_err(|e|{
        error!("Failed to delete teacher {:?}",e);
        e
    })?;
    info!("success to delete a teacher");
    Ok(HttpResponse::Ok().json(teachers))
}


pub async fn update_teacher_by_id(data:web::Data<AppState>,teacher:web::Json<UpdateTeacher>,params:web::Path<i32>) -> Result<HttpResponse,MyError>{
    let id = params.into_inner();
    let teachers = update_teacher_by_id_db(&data.db_pool, teacher.try_into()?,id).await.map_err(|e|{
        error!("Failed to update a teacher {:?}",e);
        e
    })?;
    info!("success to update a teacher,id:{}",teachers.id);
    Ok(HttpResponse::Ok().json(teachers))
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
    async fn post_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let data = web::Data::new(AppState::new(pool));
        let teacher = web::Json(CreateTeacher{
            name: "post_test".into(),
            picture_url: "http://test_teacher".into(),
            profile: "post_test".into()
        });
        let res  = post_new_teacher(data, teacher).await.unwrap();
        assert_eq!(res.status(),StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let data = web::Data::new(AppState::new(pool));
        let res = get_all_teachers(data).await.unwrap();
        assert_eq!(res.status(),StatusCode::OK)
    }


    #[actix_rt::test]
    async fn delete_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let data = web::Data::new(AppState::new(pool));
        let params = web::Path::from(5000000);
        let res = delete_teacher_by_id(data, params).await;
        match res{
            Ok(_) => println!("some wrong in delete teacher test"),
            Err(e) => assert_eq!(e.status_code(),StatusCode::NOT_FOUND)
        }
    }
}