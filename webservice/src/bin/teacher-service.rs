use std::{env, io};
use std::sync::Mutex;
use actix_web::{web, App,HttpServer};
use actix_web::cookie::time::format_description::modifier::SubsecondDigits::One;
use crate::routers::{general_routes, course_routes, teacher_routes};
use crate::state::AppState;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::errors::MyError;

#[path= "../handlers/mod.rs"]
mod handlers;
#[path="../routers.rs"]
mod routers;
#[path="../state.rs"]
mod state;
#[path= "../models/mod.rs"]
mod models;
#[path="../dbaccess/mod.rs"]
mod dbaccess;
#[path="../errors.rs"]
mod errors;

#[actix_rt::main]
async fn main() ->io::Result<()>{
    dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL 没有在.env文件里设置");
    let db_pool=PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data=web::Data::new(AppState{health_check_response:"I'm OK".to_string(),visit_count:Mutex::new(0),db:db_pool});
    let app=move||{
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {MyError::InvalidInput("Please provide valid Json input".to_string()).into()}))
            .configure(general_routes).configure(course_routes).configure(teacher_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await?;
    Ok(())
}