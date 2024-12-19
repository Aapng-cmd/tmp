use diesel::prelude::*;

use crate::pkg::db;
use crate::error::AppError;
use crate::schema::users;

#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn create(username: String, password: String) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = diesel::insert_into(users::table).values((
                    users::username.eq(username),
                    users::password.eq(password),
                ))
                .get_result(&mut conn);

                match res {
                    Ok(user) => Ok(user),
                    Err(_) => Err(AppError::new("Failed to create user".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }
    pub fn find(username: String) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = users::table
                    .filter(users::username.eq(username))
                    .first(&mut conn);
                match res {
                    Ok(user) => Ok(user),
                    Err(e) => Err(AppError::new(e.to_string()))
                }
            }
            Err(e) => Err(e)
        }
    }
    pub fn find_by_id(id: i32) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = users::table
                    .filter(users::id.eq(id))
                    .first(&mut conn);
                match res {
                    Ok(user) => Ok(user),
                    Err(e) => Err(AppError::new(e.to_string()))
                }
            }
            Err(e) => Err(e)
        }
    }  
    pub fn find_all(page: u32) -> Result<Vec<Self>, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let offset: i64 = ((page - 1) * 100).into();
                let res = users::table
                    .offset(offset)
                    .limit(100)
                    .load::<Self>(&mut conn);
                match res {
                    Ok(users) => Ok(users),
                    Err(e) => Err(AppError::new(e.to_string()))
                }
            }
            Err(e) => Err(e)
        }
    }
}