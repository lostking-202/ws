use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug,Clone,sqlx::FromRow)]
pub struct Teacher{
    pub id:i32,
    pub name:String,
    pub picture_url:String,
    pub profile:String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct CreateTeacher{
    pub name:String,
    pub picture_url:String,
    pub profile:String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct UpdateTeacher{
    pub name:Option<String>,
    pub picture_url:Option<String>,
    pub profile:Option<String>
}

impl From<web::Json<CreateTeacher>> for CreateTeacher{
    fn from(new_teacher: Json<CreateTeacher>) -> CreateTeacher {
        CreateTeacher{
            name:new_teacher.name.clone(),
            picture_url:new_teacher.picture_url.clone(),
            profile:new_teacher.profile.clone()
        }
    }
}


impl From<web::Json<UpdateTeacher>> for UpdateTeacher{
    fn from(update_teacher: Json<UpdateTeacher>) -> UpdateTeacher {
        UpdateTeacher{
            name:update_teacher.name.clone(),
            picture_url:update_teacher.picture_url.clone(),
            profile:update_teacher.profile.clone()
        }
    }
}