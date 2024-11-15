use actix_web::{web, App, HttpResponse};
use crate::dbaccess::teacher::{delete_teacher_db, get_all_teachers_db, get_teacher_details_db, post_new_teacher_db, update_teacher_details_db};
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

pub async fn get_all_teachers(app_state: web::Data<AppState>) ->Result<HttpResponse,MyError>{
    get_all_teachers_db(&app_state.db).await.map(|teachers|HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(app_state:web::Data<AppState>,params:web::Path<i32>)->Result<HttpResponse,MyError>{
    get_teacher_details_db(&app_state.db,params.into_inner()).await.map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(app_state: web::Data<AppState>,new_teacher:web::Json<CreateTeacher>)->Result<HttpResponse,MyError>{
    post_new_teacher_db(&app_state.db,CreateTeacher::from(new_teacher)).await.map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(app_state:web::Data<AppState>,params:web::Path<i32>,update_teacher: web::Json<UpdateTeacher>)->Result<HttpResponse,MyError>{
    update_teacher_details_db(&app_state.db,params.into_inner(),UpdateTeacher::from(update_teacher)).await.map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(app_state:web::Data<AppState>,params:web::Path<i32>)->Result<HttpResponse,MyError>{
    delete_teacher_db(&app_state.db,params.into_inner()).await.map(|msg|HttpResponse::Ok().json(msg))
}