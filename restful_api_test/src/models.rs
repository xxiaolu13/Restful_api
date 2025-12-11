use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};

#[derive(Clone,Deserialize,Serialize,Debug)]
pub struct Course{
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course{
    fn from(data:web::Json<Course>)->Course{
        Self { teacher_id: (data.teacher_id), id: (data.id), name: (data.name.clone()), time: (data.time) }
    }
}

