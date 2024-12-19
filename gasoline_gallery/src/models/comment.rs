use actix_web::Result;
use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use serde::Serialize;

use crate::schema::users;
use crate::{error::AppError, pkg::db, schema::comments};

#[derive(Debug, Serialize, Queryable)]
pub struct CommentResponse {
    pub id: i32,
    pub text: String,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, Identifiable)]
#[diesel(belongs_to(User, foreign_key = from_id))]
#[diesel(belongs_to(Post, foreign_key = post_id))]
pub struct Comment {
    pub id: i32,
    pub text: String,
    pub from_id: i32,
    pub post_id: i32,
    pub created_at: NaiveDateTime,
}

impl Comment {
    pub fn create_one(from_id: i32, post_id: i32, text: String) -> Result<Self, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let res = diesel::insert_into(comments::table)
                    .values((
                        comments::from_id.eq(from_id),
                        comments::post_id.eq(post_id),
                        comments::text.eq(text),
                    ))
                    .get_result(&mut conn);
                match res {
                    Ok(comment) => Ok(comment),
                    Err(_) => Err(AppError::new("Failed to create comment".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn get_all_by_post_id(post_id: i32, page: u32) -> Result<Vec<CommentResponse>, AppError> {
        let conn_res = db::connection();
        match conn_res {
            Ok(mut conn) => {
                let offset: i64 = ((page - 1) * 5).into();
                let res = comments::table
                    .inner_join(users::table.on(users::id.eq(comments::from_id)))
                    .select((
                        comments::id,
                        comments::text,
                        users::username,
                        comments::created_at,
                    ))
                    .filter(comments::post_id.eq(post_id))
                    .order(comments::created_at.desc())
                    .offset(offset)
                    .limit(5)
                    .load::<CommentResponse>(&mut conn);
                match res {
                    Ok(comments) => Ok(comments),
                    Err(_) => Err(AppError::new("Failed to get comments".to_string())),
                }
            }
            Err(e) => Err(e)
        }
    }
}