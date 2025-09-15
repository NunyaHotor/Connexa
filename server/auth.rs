use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, http::request::Parts};
use axum::headers::Authorization;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error as JwtError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user/recipient id)
    pub exp: usize,
}

pub struct AuthenticatedUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<axum::headers::authorization::Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| "Missing or invalid Authorization header".to_string())?;

        let token = bearer.token();
        let key = DecodingKey::from_secret(b"your-secret-key"); // Use a secure key!
        let validation = Validation::new(Algorithm::HS256);

        let token_data: TokenData<Claims> = decode::<Claims>(token, &key, &validation)
            .map_err(|e| format!("Invalid token: {}", e))?;

        Ok(AuthenticatedUser {
            user_id: token_data.claims.sub,
        })
    }
}