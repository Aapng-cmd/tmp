use actix_web::{middleware::from_fn, web, App, HttpServer};
use gasoline_gallery::{
    controllers::{
        auth::{login, register}, 
        post::{accept_request, create_post, get_comments, get_post, get_post_image, get_posts, get_requests, get_user_posts, reject_request, request_access, send_comment}, user::{get_current_user, get_user_profile, get_users}
    }, middlewares::auth::auth_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server has been started");
    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(auth_middleware))
            .service(web::scope("/api")
                .service(get_users)
                .service(get_current_user)
                .service(login)
                .service(register)
                .service(create_post)
                .service(get_posts)
                .service(get_post)
                .service(get_post_image)
                .service(get_comments)
                .service(request_access)
                .service(accept_request)
                .service(reject_request)
                .service(get_requests)
                .service(send_comment)
                .service(get_user_posts)
                .service(get_user_profile)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}