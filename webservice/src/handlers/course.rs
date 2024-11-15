use actix_web::{web, HttpResponse};
use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::state::AppState;
use crate::dbaccess::course::{get_course_details_db,get_courses_for_teacher_db,post_new_course_db,delete_course_db,update_course_details_db};

pub async fn new_course(new_course:web::Json<CreateCourse>,app_state: web::Data<AppState>)->Result<HttpResponse,MyError>{
    println!("Received new courses");
    post_new_course_db(&app_state.db, new_course.try_into()?).await.map(|course: Course|HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(app_state: web::Data<AppState>,params:web::Path<i32>)->Result<HttpResponse,MyError>{
    let teacher_id=params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id).await
        .map(|course|HttpResponse::Ok().json(course))

}

pub async fn get_course_detail(app_state: web::Data<AppState>,params:web::Path<(i32,i32)>)->Result<HttpResponse,MyError>{
    let (teacher_id,course_id)=params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id).await.map(|course: Course|HttpResponse::Ok().json(course))

}

pub async fn delete_course(app_state: web::Data<AppState>,params:web::Path<(i32,i32)>)->Result<HttpResponse,MyError>{
    let (teacher_id,course_id)=params.into_inner();
    delete_course_db(&app_state.db,teacher_id,course_id).await.map(|msg:String|HttpResponse::Ok().json(msg))
}

pub async fn update_course_details(update_course:web::Json<UpdateCourse>,app_state: web::Data<AppState>,params:web::Path<(i32,i32)>)->Result<HttpResponse,MyError>{
    let (teacher_id,course_id)=params.into_inner();
    update_course_details_db(&app_state.db, teacher_id,course_id,update_course.try_into()?).await
        .map(|course|HttpResponse::Ok().json(course))

}
