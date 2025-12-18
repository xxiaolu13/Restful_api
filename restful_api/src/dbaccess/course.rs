use crate::models::course::*;
// use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use crate::errors::*;
// query!
pub async fn get_courses_by_teacher_id_db(pool:&PgPool,teacher_id:i32)->Result<Vec<Course>,MyError>{
    let rows = 
    sqlx::query_as!(Course,"select * from course where teacher_id = $1",teacher_id)
    .fetch_all(pool).await?;
    match rows.len(){
        0 => Err(MyError::NotFound(format!("get course by teacherid:{} not found",teacher_id))),
        _ => Ok(rows)
    }
    
}


// query_as!
pub async fn get_courses_by_id_db(pool:&PgPool,id:i32)->Result<Course, MyError>{
    let row = sqlx::query_as!(
        Course,
        "select * from course where id = $1",
        id
    ).fetch_optional(pool).await?;
    if let Some(course) = row{
        Ok(course)
    }else{
        Err(MyError::NotFound(format!("get course by id:{} not found",id)))
    }
}


pub async fn get_all_courses_db(pool: &PgPool)-> Result<Vec<Course>,MyError>{
    let all_course = 
    sqlx::query_as!(Course,"select * from course").fetch_all(pool).await?;
    match all_course.len(){
        0 => Err(MyError::NotFound("get all course not found".into())),
        _ => Ok(all_course)
    }
}


pub async fn post_new_course_db(pool: &PgPool,newcourse: CreateCourse)-> Result<Course, MyError>{
    let course = sqlx::query_as!(
        Course,
        "insert into course (teacher_id,name,description,format,structure,duration,price,language,level) values($1,$2,$3,$4,$5,$6,$7,$8,$9) returning *",
        newcourse.teacher_id,
        newcourse.name,
        newcourse.description,
        newcourse.format,
        newcourse.structure,
        newcourse.duration,
        newcourse.price,
        newcourse.language,
        newcourse.level
    ).fetch_one(pool).await?;
    Ok(course)
}


pub async fn delete_course_by_teacherid_id_db(pool:&PgPool,teacher_id:i32,id:i32) -> Result<String,MyError>{
    let del_msg = sqlx::query!("delete from course where teacher_id = $1 and id = $2",teacher_id,id)
    .execute(pool).
    await?;
    Ok(format!("Success Deleted {:?} record",del_msg))
}


// pub struct UpdateCourse{
//     pub name: Option<String>,
//     pub description: Option<String>,
//     pub format: Option<String>,
//     pub structure: Option<String>,
//     pub duration: Option<String>,
//     pub price: Option<i32>,
//     pub language: Option<String>,
//     pub level: Option<String>,
// }


// fn supercheck<T>(a: Option<T>, b: Option<T>) -> Option<T> {
//     if let Some(c) = a {
//         Some(c)
//     } else {
//         b
//     }
// }
fn supercheck<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.or(b)
}

pub async fn update_course_by_teacherid_id_db(pool: &PgPool,teacher_id:i32,id:i32,updatecourse:UpdateCourse) -> Result<Course,MyError>{
    let course_current_row = 
    sqlx::query_as!(Course,"select * from course where teacher_id = $1 and id = $2",teacher_id,id)
    .fetch_one(pool)
    .await.map_err(|_err| MyError::NotFound(format!("updating course not found by teacherid:{} and id:{}",teacher_id,id)));
    let course: Course = course_current_row?;
    // let name = supercheck(updatecourse.name,course.name);
    let name = if let Some(name) = updatecourse.name{
        name
    }else{
        course.name
    };
    let description = supercheck(updatecourse.description,course.description);
    let format = supercheck(updatecourse.format,course.format);
    let structure = supercheck(updatecourse.structure,course.structure);
    let duration = supercheck(updatecourse.duration,course.duration);
    let price = supercheck(updatecourse.price,course.price);
    let language = supercheck(updatecourse.language,course.language);
    let level = supercheck(updatecourse.level,course.level);
    
    let row = sqlx::query_as!(
        Course,
        "update course set name = $1,description = $2,format = $3,structure = $4,duration = $5,price = $6,language = $7,level = $8 where teacher_id = $9 and id = $10 returning *",
        name,description,format,structure,duration,price,language,level,teacher_id,id
    ).fetch_one(pool).await?;

    Ok(row)
}
