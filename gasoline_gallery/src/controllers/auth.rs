use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{config::config::CONFIG, error::AppError, models::user::User, pkg::jwt, utils::utils};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password1: String,
    password2: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String
}

#[derive(Serialize)]
pub struct RegisterResponse {
    id: i32
}

#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    let login_req: LoginRequest = req.into_inner();
    let user_res = User::find(login_req.username);
    match user_res {
        Ok(user) => {
            let hash_password = utils::hash_string(login_req.password, CONFIG.hash_salt.clone());

            if hash_password != user.password {
                return HttpResponse::BadRequest().json(AppError::new(String::from("Invalid credentials")));
            }

            let token = jwt::generate_token(user.id, CONFIG.jwt_secret_key.clone());
            HttpResponse::Ok().json(LoginResponse { token })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(e)
        }
    }
}

#[post("/register")]
pub async fn register(req: web::Json<RegisterRequest>) -> HttpResponse {
    let register_req: RegisterRequest = req.into_inner();

    if register_req.password1 != register_req.password2 {
        return HttpResponse::BadRequest().json(AppError::new(String::from("Passwords do not match")));
    }

    let hashed_password = utils::hash_string(register_req.password1, CONFIG.hash_salt.clone());

    match User::create(register_req.username, hashed_password) {
        Ok(user) => HttpResponse::Created().json(RegisterResponse {
            id: user.id
        }),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}