use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error, HttpMessage
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{config::config::CONFIG, pkg::jwt::Claims};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let path = req.path();

    if path == "/api/login" || path == "/api/register" {
        return next.call(req).await;
    }
    
    match req.cookie("gasoline-token") {
        Some(gasoline_access) => {
            let token = gasoline_access.value();
            match decode::<Claims>(
                token, 
                &DecodingKey::from_secret(CONFIG.jwt_secret_key.as_bytes()), 
                &Validation::new(Algorithm::HS256)
            ) {
                Ok(token_data) => {
                    let claims = token_data.claims;
                    let user_id = claims.sub;
                    req.extensions_mut().insert(user_id);
                    next.call(req).await
                }
                Err(e) => {
                    println!("Invalid token: {}", e);
                    Err(Error::from(actix_web::error::ErrorUnauthorized("{\"message\":\"Invalid token\"}")))
                }
            }
        }
        None => Err(Error::from(actix_web::error::ErrorUnauthorized("{\"message\":\"Missing token\"}")))
    }
}