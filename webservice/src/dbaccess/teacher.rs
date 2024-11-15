use sqlx::PgPool;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_all_teachers_db(pool:&PgPool) ->Result<Vec<Teacher>,MyError>{
    let rows:Vec<Teacher>=sqlx::query_as!(Teacher,r"select * from teacher").fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_teacher_details_db(pool:&PgPool,teacher_id:i32) ->Result<Teacher,MyError>{
    Ok(sqlx::query_as!(Teacher,"select * from teacher where id=$1",teacher_id).fetch_one(pool).await?)
}

pub async fn post_new_teacher_db(pool:&PgPool,new_teacher:CreateTeacher)->Result<Teacher,MyError>{
    Ok(sqlx::query_as!(Teacher,"insert into teacher(name,picture_url,profile)values($1,$2,$3) returning id,name,picture_url,profile",
    new_teacher.name,new_teacher.picture_url,new_teacher.profile).fetch_one(pool).await?)
}

pub async fn update_teacher_details_db(pool:&PgPool,id:i32,update_teacher: UpdateTeacher)->Result<Teacher,MyError>{

    let current_teacher_row=sqlx::query_as!(Teacher,"select * from teacher where id=$1",id).fetch_one(pool).await
        .map_err(|_err|MyError::NotFound("Teacher Id not found".into()))?;

    let name:String= if let Some(name)=update_teacher.name{
        name
    }else{
        current_teacher_row.name
    };

    let picture_url:String= if let Some(picture_url)=update_teacher.picture_url{
        picture_url
    }else{
        current_teacher_row.picture_url
    };

    let profile:String= if let Some(profile)=update_teacher.profile{
        profile
    }else{
        current_teacher_row.profile
    };

    let teacher_row=sqlx::query_as!(Teacher,"update teacher set name=$1,picture_url=$2,profile=$3 where id=$4 returning id,name,picture_url,profile",name,picture_url,profile,id).fetch_one(pool).await;
    if let Ok(teacher)=teacher_row{
        Ok(teacher)
    }else{
        Err(MyError::NotFound("Teacher Id not found".into()))
    }
}

pub async fn delete_teacher_db(pool:&PgPool,teacher_id:i32)->Result<String,MyError>{
    let row=sqlx::query!("delete from teacher where id=$1",teacher_id).execute(pool).await.map_err(|e| MyError::DBError("unable to delete teacher".into()));
    Ok(format!("Deleted {:?} record",row))
}