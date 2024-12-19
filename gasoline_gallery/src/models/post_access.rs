use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, BoolExpressionMethods};
use diesel_derive_enum::DbEnum;

use crate::schema::posts;
use crate::{error::AppError, pkg::db, schema::{post_accesses, users}};

use super::user::User;

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AccessType"]
pub enum AccessType {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Queryable)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Post, foreign_key = post_id))]
pub struct PostAccess {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub access_type: AccessType,
    pub created_at: NaiveDateTime,
}

impl PostAccess {
    pub fn create_one(user_id: i32, post_id: i32) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = diesel::insert_into(post_accesses::table).
                    values((
                        post_accesses::user_id.eq(user_id),
                        post_accesses::post_id.eq(post_id),
                        post_accesses::access_type.eq(AccessType::Pending),
                    ))
                    .get_result(&mut conn);
                match res {
                    Ok(post_access) => Ok(post_access),
                    Err(_) => Err(AppError::new("Failed to create post access".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn get_users_by_post_id(post_id: i32) -> Result<Vec<User>, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = post_accesses::table
                    .inner_join(users::table.on(users::id.eq(post_accesses::user_id)))
                    .select(users::all_columns)
                    .filter(post_accesses::post_id.eq(post_id))
                    .load::<User>(&mut conn);
                match res {
                    Ok(users) => Ok(users),
                    Err(_) => Err(AppError::new("Failed to get users by post id".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn update_access(access_id: i32, status: AccessType) -> Option<AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = diesel::update(post_accesses::table)
                    .filter(post_accesses::id.eq(access_id))
                    .set(post_accesses::access_type.eq(status))
                    .execute(&mut conn);
                match res {
                    Ok(_) => None,
                    Err(_) => Some(AppError::new("Failed to update post access request".to_string())),
                }
            }
            Err(e) => Some(e)
        }
    }

    pub fn exists_by_filename(user_id: i32, filename: String) -> bool {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = post_accesses::table
                    .left_join(posts::table)
                    .filter(
                        posts::filename.eq(filename).and(post_accesses::user_id.eq(user_id))
                    )
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

    pub fn find_all_by_post_id(post_id: i32) -> Result<Vec<PostAccess>, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = post_accesses::table.filter(
                    post_accesses::post_id.eq(post_id)
                ).load(&mut conn);
                match res {
                    Ok(post_accesses) => Ok(post_accesses),
                    Err(e) => {
                        println!("{}", e);
                        Err(AppError::new("Failed to find all post access requests by post id".to_string()))
                    }
                }
            }
            Err(e) => Err(e)
        }
    }
}