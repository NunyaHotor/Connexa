use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, http::request::Parts};
use axum::headers::Authorization;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
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
        // Allow insecure auth for local testing if ALLOW_INSECURE_AUTH is set.
        let insecure = std::env::var("ALLOW_INSECURE_AUTH")
            .ok()
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        // Try to extract Bearer header if present
        let header_result = TypedHeader::<Authorization<axum::headers::authorization::Bearer>>::from_request_parts(parts, _state).await;

        if insecure {
            // In insecure mode, accept any token or even no header and use a fixed test user id.
            if let Ok(TypedHeader(Authorization(bearer))) = header_result {
                return Ok(AuthenticatedUser { user_id: bearer.token().to_string() });
            } else {
                return Ok(AuthenticatedUser { user_id: "test-user".to_string() });
            }
        }

        // Secure mode: require a valid JWT
        let TypedHeader(Authorization(bearer)) = header_result
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