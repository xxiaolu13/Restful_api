use sqlx::postgres::PgPool;
use crate::errors::*;
use crate::models::teacher::*;
use tracing::{info, warn, debug, error};

pub async fn get_all_teachers_db(pool:&PgPool)-> Result<Vec<Teacher>, MyError>{
    let rows = sqlx::query_as!(
        Teacher,
        "select * from teacher"
    ).fetch_all(pool).await?;
    match rows.len(){
        0 => Err(MyError::NotFound("get all teacher not found".into())),
        _ => Ok(rows)
    }
}


pub async fn get_teacher_by_id_db(pool:&PgPool,id:i32) -> Result<Teacher,MyError>{
    let row = sqlx::query_as!(
        Teacher,
        "select * from teacher where id = $1",
        id
    ).fetch_optional(pool).await?;
    if let Some(e) = row{
        Ok(e)
    }else{
        Err(MyError::NotFound(format!("teacher with id:{} not found",id)))
    }
}


pub async fn post_new_teacher_db(pool:&PgPool,newteacher:CreateTeacher) -> Result<Teacher,MyError>{
    let row = sqlx::query_as!(
        Teacher,
        "insert into teacher (name,picture_url,profile) values($1,$2,$3) returning *",
        newteacher.name,
        newteacher.picture_url,
        newteacher.profile
    ).fetch_one(pool).await?;
    Ok(row)
}


pub async fn delete_teacher_by_id_db(pool:&PgPool,id:i32) -> Result<String,MyError>{
    let del_msg = sqlx::query!(
        "delete from teacher where id = $1",
        id
    ).execute(pool).await?;
    Ok(format!("Success Deleted {:?} record",del_msg))
}


pub async fn  update_teacher_by_id_db(pool:&PgPool,newteacher:UpdateTeacher,id:i32) -> Result<Teacher,MyError>{
    let check = sqlx::query_as!(Teacher,"select * from teacher where id = $1",id)
    .fetch_one(pool).await
    .map_err(|_err| MyError::NotFound(format!("updating teacher not found by id:{}",id)));
    let teacher = check?;
    let name = if let Some(e) = newteacher.name{
        e
    }else{
        teacher.name
    };
    let picture_url = if let Some(e) = newteacher.picture_url{
        e
    }else{
        teacher.picture_url
    };
    let profile = if let Some(e) = newteacher.profile{
        e
    }else{
        teacher.profile
    };
    let row = sqlx::query_as!(
        Teacher,
        "update teacher set name = $1,picture_url= $2,profile=$3 where id = $4 returning *",
        name,picture_url,profile,teacher.id
    ).fetch_one(pool).await?;
    Ok(row)
}