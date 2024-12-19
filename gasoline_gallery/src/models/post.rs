use diesel::{prelude::{Identifiable, Insertable, Queryable}, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use chrono::{NaiveDateTime, Utc};

use crate::{error::AppError, pkg::db, schema::{post_accesses, posts}};

#[derive(Debug, Insertable, Queryable, Identifiable)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Post {
    pub id: i32,
    pub secret_key: Option<String>,
    pub filename: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

impl Post {
    pub fn new_empty() -> Self {
        Post {
            id: 0,
            secret_key: None,
            filename: String::new(),
            user_id: 0,
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn create_one(user_id: i32, secret_key: Option<String>, filename: String) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = diesel::insert_into(posts::table)
                    .values((
                        posts::user_id.eq(user_id),
                        posts::secret_key.eq(secret_key),
                        posts::filename.eq(filename),
                    ))
                    .get_result(&mut conn);
                match res {
                    Ok(post) => Ok(post),
                    Err(_) => Err(AppError::new("Failed to create post".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn get_all_by_user_id(user_id: i32, page: u32) -> Result<Vec<Self>, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let offset: i64 = ((page - 1) * 10).into();
                let res = posts::table
                    .filter(posts::user_id.eq(user_id))
                    .order(posts::created_at.desc())
                    .offset(offset)
                    .limit(10)
                    .load::<Self>(&mut conn);
                match res {
                    Ok(posts) => Ok(posts),
                    Err(_) => Err(AppError::new("Failed to get posts".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_by_id(id: i32) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = posts::table
                    .filter(posts::id.eq(id))
                    .get_result(&mut conn);
                match res {
                    Ok(post) => Ok(post),
                    Err(_) => Err(AppError::new("Failed to find post by".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_one_by_access_id(access_id: i32) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = posts::table
                    .left_join(post_accesses::table)
                    .filter(post_accesses::id.eq(access_id))
                    .select(posts::all_columns)
                    .get_result(&mut conn);
                match res {
                    Ok(post) => Ok(post),
                    Err(_) => Err(AppError::new("Failed to find post by access_id".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_one_by_filename(filename: String) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = posts::table
                    .filter(posts::filename.eq(filename))
                    .get_result(&mut conn);
                match res {
                    Ok(post) => Ok(post),
                    Err(_) => Err(AppError::new("Failed to find post by filename".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn exists(user_id: i32, filename: String) -> bool {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = posts::table
                    .filter(posts::user_id.eq(user_id).and(posts::filename.eq(filename)))
                    .count()
                    .get_result::<i64>(&mut conn);
                match res {
                    Ok(count) => count > 0,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}