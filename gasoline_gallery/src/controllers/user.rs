use actix_web::{get, web, HttpRequest, HttpMessage, HttpResponse};
use serde::Serialize;
use crate::models::user::User;

use super::post::QueryData;

#[derive(Serialize)]
struct UserResponse {
    pub id: i32,
    pub username: String
}

#[get("/users")]
pub async fn get_users(query: web::Query<QueryData>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    match User::find_all(page) {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users.iter()
                .map(|user| UserResponse {
                    id: user.id,
                    username: user.username.clone(),
                })
                .collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[get("me")]
pub async fn get_current_user(req: HttpRequest) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    match User::find_by_id(user_id) {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            username: user.username,
        }),
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_profile(path: web::Path<i32>) -> HttpResponse {
    let user_id = path.into_inner();
    match User::find_by_id(user_id) {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            username: user.username,
        }),
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}