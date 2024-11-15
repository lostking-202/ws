use actix_web::web;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};
use crate::errors::MyError;

#[derive(Serialize,Debug,Clone,sqlx::FromRow)]
pub struct Course{
    pub id:i32,
    pub teacher_id:i32,
    pub name:String,
    pub time:Option<NaiveDateTime>,
    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}


#[derive(Deserialize,Debug,Clone)]
pub struct CreateCourse{
    pub teacher_id:i32,
    pub name:String,

    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}

/*impl From<web::Json<CreateCourse>> for CreateCourse{
    fn from(course: Json<CreateCourse>) -> Self {
        CreateCourse{
            teacher_id:course.teacher_id,
            name:course.name.clone(),
            description:course.description.clone(),
            format:course.format.clone(),
            structure:course.structure.clone(),
            duration:course.structure.clone(),
            price:course.price,
            language:course.language.clone(),
            level:course.level.clone()
        }
    }
}*/

impl TryFrom<Json<CreateCourse>> for CreateCourse{
    type Error =MyError;

    fn try_from(course: Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse{
            teacher_id:course.teacher_id,
            name:course.name.clone(),
            description:course.description.clone(),
            format:course.format.clone(),
            structure:course.structure.clone(),
            duration:course.structure.clone(),
            price:course.price,
            language:course.language.clone(),
            level:course.level.clone()
        })
    }
}

#[derive(Deserialize,Debug,Clone)]
pub struct UpdateCourse{
    pub name:Option<String>,

    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}


impl TryFrom<Json<UpdateCourse>> for UpdateCourse{
    type Error =MyError;

    fn try_from(course: Json<UpdateCourse>) -> Result<Self, Self::Error> {
        Ok(UpdateCourse{
            name:course.name.clone(),
            description:course.description.clone(),
            format:course.format.clone(),
            structure:course.structure.clone(),
            duration:course.structure.clone(),
            price:course.price,
            language:course.language.clone(),
            level:course.level.clone()
        })
    }
}

#[test]
fn test(){
    let num = 1.23457e10;
    let formatted = format!("{:.2}", num); // 输出两位小数，不使用科学计数法
    let formatted_scientific = format!("{:e}", num); // 强制使用科学计数法
    let formatted_scientific_two = format!("{:.2e}", num); // 使用科学计数法并保留两位小数

    println!("{}", formatted);
    println!("{}", formatted_scientific);
    println!("{}", formatted_scientific_two);
}