use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;
use std::io::{Read, Write};
use actix_multipart::form::{text::Text, tempfile::TempFile, MultipartForm};

use crate::error::AppError;
use crate::models::comment::Comment;
use crate::models::post::Post;
use crate::models::post_access::{AccessType, PostAccess};
use crate::utils::utils;

#[derive(Serialize)]
struct PostResponse {
    id: i32,
    filename: String,
    is_private: bool,
    created_at: String,
}

#[derive(Deserialize)]
pub struct QueryData {
    pub page: Option<u32>
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "10MB")]
    image: TempFile,
    is_private: Text<bool>,
}

#[derive(Debug, Serialize)]
struct PostAccessResponse {
    id: i32,
    user_id: i32,
    created_at: String
}

#[derive(Debug, Deserialize)]
struct SendCommentRequest {
    text: String
}

#[derive(Serialize)]
struct RequestAccessResponse {
    id: i32
}

#[post("/posts")]
pub async fn create_post(MultipartForm(form): MultipartForm<UploadForm>, req: HttpRequest) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let mut post: Post = Post::new_empty();
    post.user_id = user_id;
    let filename = Uuid::new_v4().to_string();       
    post.filename = filename.clone();
    let filepath = format!("/uploads/{}.jpg", &filename);
    let mut file = web::block(|| std::fs::File::create(filepath))
        .await.unwrap().unwrap();

    let mut image_data = Vec::new();
    form.image.file.as_file().read_to_end(&mut image_data).expect("Failed to read image data");
    let is_private = form.is_private.0;
    if is_private {
        let key = utils::random_bytes(8);
        utils::encrypt_data(&mut image_data, key.clone());
        post.secret_key = Some(utils::vec_to_b64(key));
    }

    file.write_all(&image_data).expect("Failed to write to file");

    match Post::create_one(post.user_id, post.secret_key, post.filename) {
        Ok(post) => HttpResponse::Created().json(PostResponse {
            id: post.id,
            filename: post.filename,
            is_private,
            created_at: post.created_at.to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[get("/posts")]
pub async fn get_posts(req: HttpRequest, query: web::Query<QueryData>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let user_id = *req.extensions().get::<i32>().unwrap();
    match Post::get_all_by_user_id(user_id, page) {
        Ok(posts) => {
            let posts_response: Vec<PostResponse> = posts
                .into_iter()
                .map(|post| PostResponse {
                    id: post.id,
                    filename: post.filename,
                    is_private: post.secret_key.is_some(),
                    created_at: post.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(posts_response)
        }
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[get("/users/{user_id}/posts")]
pub async fn get_user_posts(path: web::Path<i32>, query: web::Query<QueryData>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let user_id = path.into_inner();
    match Post::get_all_by_user_id(user_id, page) {
        Ok(posts) => {
            let posts_response: Vec<PostResponse> = posts
                .into_iter()
                .map(|post| PostResponse {
                    id: post.id,
                    filename: post.filename,
                    is_private: post.secret_key.is_some(),
                    created_at: post.created_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(posts_response)
        }
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[get("/posts/{post_id}")]
pub async fn get_post(path: web::Path<i32>) -> HttpResponse {
    let post_id = path.into_inner();
    match Post::find_by_id(post_id) {
        Ok(post) => {
            HttpResponse::Ok().json(PostResponse {
                id: post.id,
                filename: post.filename,
                is_private: post.secret_key.is_some(),
                created_at: post.created_at.to_string(),
            })
        },
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[get("/uploads/{filename}")]
pub async fn get_post_image(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let filename = path.into_inner();
    if filename.contains("..") || filename.contains("/") {
        return HttpResponse::BadRequest().json(AppError::new("Illegal file name".to_string()));
    }
    let file_path = PathBuf::from(format!("/uploads/{}", filename));

    match fs::read(&file_path) {
        Ok(mut file_content) => {
            let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
            let mut filename_trunc = filename.clone();
            filename_trunc.truncate(filename.len() - 4);
            match Post::find_one_by_filename(filename_trunc.clone()) {
                Ok(post) => {
                    if PostAccess::exists_by_filename(user_id, filename_trunc) || (post.secret_key.is_some() && post.user_id == user_id) {
                        let secret_key = post.secret_key.expect("no secret key found");
                        let key = utils::b64_to_vec(secret_key);
                        utils::decrypt_data(&mut file_content, key.clone());
                    }
                }
                Err(e) => { return HttpResponse::BadRequest().json(e); },
            }
            HttpResponse::Ok()
                .content_type(mime_type)
                .body(file_content)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("/posts/{post_id}/comments")]
pub async fn send_comment(req: HttpRequest, body: web::Json<SendCommentRequest>, path: web::Path<i32>) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let post_id = path.into_inner();
    match Comment::create_one(user_id, post_id, body.text.clone()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[get("/posts/{post_id}/comments")]
pub async fn get_comments(path: web::Path<i32>, query: web::Query<QueryData>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let post_id = path.into_inner();
    match Comment::get_all_by_post_id(post_id, page) {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[post("/posts/{post_id}/accesses")]
pub async fn request_access(req: HttpRequest, path: web::Path<i32>) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let post_id = path.into_inner();

    match Post::find_by_id(post_id) {
        Ok(post) => {
            if post.user_id == user_id {
                return HttpResponse::BadRequest().json(AppError::new("cannot request access to own post".to_string()));
            }
            let duration = Utc::now().naive_utc() - post.created_at;
            if duration.num_seconds() > 30 {
                return HttpResponse::BadRequest().json(AppError::new("ability to request an access is expired".to_string()));
            }
            match PostAccess::create_one(user_id, post_id) {
                Ok(post_access) => HttpResponse::Created().json(RequestAccessResponse {
                    id: post_access.id,
                }),
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(e)
    }
}

#[post("/accesses/{access_id}/accept")]
pub async fn accept_request(req: HttpRequest, path: web::Path<i32>) -> HttpResponse {
    let user_id: i32 = *req.extensions().get::<i32>().unwrap();
    let access_id = path.into_inner();
    match Post::find_one_by_access_id(access_id) {
        Ok(post) => {
            if post.user_id != user_id {
                return HttpResponse::Forbidden().json(AppError::new("unauthorized access to accept request".to_string()));
            }
            match PostAccess::update_access(access_id, AccessType::Accepted) {
                None => HttpResponse::Ok().finish(),
                Some(e) => HttpResponse::BadRequest().json(e),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(AppError::new("Failed to find post by access_id".to_string())),
    }
}

#[post("/accesses/{access_id}/reject")]
pub async fn reject_request(req: HttpRequest, path: web::Path<i32>) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let access_id = path.into_inner();
    match Post::find_one_by_access_id(access_id) {
        Ok(post) => {
            if post.user_id != user_id {
                return HttpResponse::Forbidden().json(AppError::new("unauthorized access to accept request".to_string()));
            }
            match PostAccess::update_access(access_id, AccessType::Rejected) {
                None => HttpResponse::Ok().finish(),
                Some(e) => HttpResponse::BadRequest().json(e),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(AppError::new("Failed to find post by access_id".to_string())),
    }
}

#[get("/posts/{post_id}/requests")]
pub async fn get_requests(req: HttpRequest, path: web::Path<i32>) -> HttpResponse {
    let user_id = *req.extensions().get::<i32>().unwrap();
    let post_id = path.into_inner();
    match Post::find_by_id(post_id) {
        Ok(post) => {
            if post.user_id != user_id {
                return HttpResponse::Forbidden().json(AppError::new("unauthorized access to post requests".to_string()));
            }
            match PostAccess::find_all_by_post_id(post_id) {
                Ok(requests) => {
                    let requests_res: Vec<PostAccessResponse> = requests.into_iter().map(|request| PostAccessResponse {
                        id: request.id,
                        user_id: request.user_id,
                        created_at: request.created_at.to_string(),
                    })
                    .collect();
                    HttpResponse::Ok().json(requests_res)
                },
                Err(e) => HttpResponse::BadRequest().json(e),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(AppError::new("post not found".to_string()))
    }
}