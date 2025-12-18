use serde::{Deserialize,Serialize};
use actix_web::web;
use crate::errors::MyError;

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Teacher {
    pub id : i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String
}
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>
}


impl TryFrom<web::Json<Teacher>> for Teacher{
    type Error = MyError;
    fn try_from(data: web::Json<Teacher>) -> Result<Self,Self::Error>{
        Ok(Teacher { id: data.id, name: data.name.clone(), picture_url: data.picture_url.clone(), profile: data.profile.clone() })
    }
}

impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher{
    type Error = MyError;
    fn try_from(data: web::Json<CreateTeacher>) -> Result<Self,Self::Error>{
        Ok(CreateTeacher { name: data.name.clone(), picture_url: data.picture_url.clone(), profile: data.profile.clone() })
    }
}

impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher{
    type Error = MyError;
    fn try_from(data: web::Json<UpdateTeacher>) -> Result<Self,Self::Error>{
        Ok(UpdateTeacher { name: data.name.clone(), picture_url: data.picture_url.clone(), profile: data.profile.clone() })
    }
}