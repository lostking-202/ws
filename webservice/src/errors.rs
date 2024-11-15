use std::fmt::{Display, Formatter};
use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use actix_web::error::Error;
use sqlx::error::Error as SQLxError;

#[derive(Serialize,Debug)]
pub enum MyError{
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Serialize,Debug)]
pub struct MyErrorResponse{
    error_message:String,
}

impl MyError {
    fn error_response(&self)->String{
        match self {
            MyError::DBError(msg)=>{
                println!("Database error occurred: {:?}",msg);
                "Databse error".into()
            }
            MyError::ActixError(msg)=>{
                println!("Server error occurred: {:?}",msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg)=>{
                println!("Not Found error occurred: {:?}",msg);
                msg.into()
            }
            MyError::InvalidInput(msg)=>{
                println!("Invalid parameters received:{:?}",msg);
                msg.into()
            }
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}

impl error::ResponseError for MyError{
    fn status_code(&self) -> StatusCode {
        match self{
            MyError::DBError(msg)|MyError::ActixError(msg)|MyError::InvalidInput(msg)=>StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(msg)=>StatusCode::NOT_FOUND
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for MyError{
    fn from(err: Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError{
    fn from(err: sqlx::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}
