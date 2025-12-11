use crate::models::Course;
use std::collections::HashMap;
use super::*;
use chrono::Utc;

// health_handler
pub async fn health_handler(data: web::Data<AppState>) -> HttpResponse {
    let health_response = &data.health_response;
    let mut visit_count = data.visit_count.lock().unwrap();
    let res1 = format!("{}=>{}",health_response,&visit_count);
    *visit_count += 1;
    let mut map = HashMap::new();
    map.insert("message",res1);
    HttpResponse::Ok().json(map)
}

// Post handler 
pub async fn new_course_handler(new_course:web::Json<models::Course>,app_state:web::Data<AppState>)-> HttpResponse{
    // let course_count = app_state.courses.lock().unwrap() 
    // .iter().filter(|course|course.teacher_id == new_course.teacher_id).collect::<Vec<models::Course>>().len();
    // 如果用iter，最后Vec里面是引用
    let course_count = app_state.courses.lock().unwrap()
    .clone().into_iter()
    .filter(|course|course.teacher_id == new_course.teacher_id)
    .collect::<Vec<models::Course>>().len();

    let new_course = Course{
        teacher_id: new_course.teacher_id,
        id : Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc())
    };
    app_state.courses.lock().unwrap().push(new_course);
    let mut map = HashMap::new();
    map.insert("message","add new course in state success");
    HttpResponse::Ok().json(map)
}

// get all courses handler
pub async fn get_all_courses(app_state:web::Data<AppState>)-> HttpResponse{
    let all_course:Vec<models::Course> = app_state.courses.lock().unwrap().clone();
    if all_course.len() > 0{
        HttpResponse::Ok().json(all_course)
    }else{
        let mut map = HashMap::new();
        map.insert("message","no course in state");
        HttpResponse::Ok().json(map)
    }
}

// get courses by teacher_id handler
pub async fn get_courses_by_teacher_id(app_state:web::Data<AppState>,path: web::Path<usize>) -> HttpResponse{
    let teacher_id  =  path.into_inner();
    let course_result = app_state.courses.lock().unwrap()
    .clone().into_iter()
    .filter(|course|course.teacher_id == teacher_id)
    .collect::<Vec<models::Course>>();
    if course_result.len() > 0{
        HttpResponse::Ok().json(course_result)
    }else{
        let mut map = HashMap::new();
        map.insert("message",format!("no course with teacher_id:{}",teacher_id));
        HttpResponse::Ok().json(map)
    }
}

// get_course_by_teacherid_and_id
pub async fn get_course_by_teacherid_and_id(app_state:web::Data<AppState>,path: web::Path<(usize,usize)>)->HttpResponse{
    let (teacher_id,id) = path.into_inner();
    let course_result = app_state.courses.lock().unwrap()
    .clone().into_iter()
    .filter(|course|course.teacher_id == teacher_id && course.id == Some(id))
    .collect::<Vec<models::Course>>();
    if course_result.len() > 0{
        HttpResponse::Ok().json(course_result)
    }else{
        let mut map = HashMap::new();
        map.insert("message",format!("no course with teacher_id:{} id:{}",teacher_id,id));
        HttpResponse::Ok().json(map)
    }
}