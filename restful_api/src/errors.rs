use serde::Serialize;
use std::fmt;
use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
    Result
};
use sqlx::error::Error as SQLxError;


#[derive(Serialize,Debug)]
pub enum MyError{ // 错误类型
    DBError(String),
    ActixError(String),
    NotFound(String)
}


#[derive(Serialize,Debug)]
pub struct MyErrorResponse{
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String{
        match self {
            MyError::DBError(msg) => {
                println!("DataBase Error Occurred: {:?}",msg);
                "DataBase Error".into()
            }
            MyError::ActixError(msg) => {
                println!("ActixError Error Occurred: {:?}",msg);
                "ActixError Error".into()
            }
            MyError::NotFound(msg) => {
                println!("NotFound Error Occurred: {:?}",msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError{
    fn status_code(&self)-> StatusCode{
        match self {
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
            // MyError::ActixError(msg) | MyError::DBError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse{
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            error_message: self.error_response()
        })
    }
}

impl fmt::Display for MyError{
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(),fmt::Error>{
        write!(f,"{}",self)
    }
}

impl From<actix_web::error::Error> for MyError{
    fn from(err:actix_web::error::Error) -> Self{
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError{
    fn from(err: SQLxError) -> Self{
        MyError::DBError(err.to_string())
    }
}