use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};
use crate::errors::MyError;
use std::convert::TryFrom;

#[derive(Clone,Serialize,Debug,sqlx::FromRow)]
pub struct Course{
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
#[derive(Clone,Deserialize,Debug)]
pub struct CreateCourse{
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
#[derive(Clone,Deserialize,Debug)]
pub struct UpdateCourse{
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
impl TryFrom<web::Json<CreateCourse>> for CreateCourse{
    type Error = MyError;
    fn try_from(data: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        
        Ok(Self{
            teacher_id: data.teacher_id,
            name: data.name.clone(),
            description: data.description.clone(),
            format: data.format.clone(),
            structure: data.structure.clone(),
            duration: data.duration.clone(),
            price: data.price,
            language: data.language.clone(),
            level: data.level.clone(),
        })
    }
}
impl TryFrom<web::Json<UpdateCourse>> for UpdateCourse{
    type Error = MyError;
    fn try_from(data: web::Json<UpdateCourse>) -> Result<Self, Self::Error>{
        Ok(Self{
            name: data.name.clone(),
            description: data.description.clone(),
            format: data.format.clone(),
            structure: data.structure.clone(),
            duration: data.duration.clone(),
            price: data.price,
            language: data.language.clone(),
            level: data.level.clone(),
        })
    }
}

