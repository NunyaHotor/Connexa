
use axum::{
    extract::State,
    routing::get,
    Json,
    Router,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::auth::AuthenticatedUser;

#[derive(Debug, Serialize, Deserialize)]
struct JitsiClaims {
    aud: String,
    iss: String,
    sub: String,
    exp: i64,
    nbf: i64,
    context: JitsiContext,
}

#[derive(Debug, Serialize, Deserialize)]
struct JitsiContext {
    user: JitsiUser,
    features: JitsiFeatures,
}

#[derive(Debug, Serialize, Deserialize)]
struct JitsiUser {
    id: String,
    name: String,
    avatar: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JitsiFeatures {
    livestreaming: bool,
    recording: bool,
    transcription: bool,
    #[serde(rename = "outbound-call")]
    outbound_call: bool,
}

pub fn generate_jitsi_token(user_id: &str, user_name: &str, user_avatar: &str, user_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let app_id = "connexa";
    let app_secret = "my_jitsi_app_secret";

    let claims = JitsiClaims {
        aud: "jitsi".to_string(),
        iss: app_id.to_string(),
        sub: "*".to_string(),
        exp: (Utc::now() + Duration::hours(3)).timestamp(),
        nbf: (Utc::now() - Duration::minutes(1)).timestamp(),
        context: JitsiContext {
            user: JitsiUser {
                id: user_id.to_string(),
                name: user_name.to_string(),
                avatar: user_avatar.to_string(),
                email: user_email.to_string(),
            },
            features: JitsiFeatures {
                livestreaming: true,
                recording: true,
                transcription: true,
                outbound_call: true,
            },
        },
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(app_secret.as_ref()))?;
    Ok(token)
}


async fn get_sfu_token(
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
) -> Result<Json<String>, String> {
    // In a real application, you would fetch the user's name, avatar, and email from a database.
    let user_name = "test_user";
    let user_avatar = "";
    let user_email = "test_user@example.com";

    let token = generate_jitsi_token(&user_id, user_name, user_avatar, user_email)
        .map_err(|e| e.to_string())?;
    Ok(Json(token))
}

pub fn sfu_router() -> Router {
    Router::new()
        .route("/sfu/token", get(get_sfu_token))
}
