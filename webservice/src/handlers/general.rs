use actix_web::{web, HttpResponse};
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) ->HttpResponse{
    let health_check_response=&app_state.health_check_response;
    let mut visit_count=app_state.visit_count.lock().unwrap();
    *visit_count+=1;
    let response=format!("{} {} times",health_check_response,visit_count);
    HttpResponse::Ok().json(&response)
}