use eyre::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    pub username: String,
}

pub fn create_token(secret: &str, username: &str) -> Result<String> {
    let user_claim = UserClaim {
        username: username.to_owned(),
    };
    let headers = Header::new(jsonwebtoken::Algorithm::HS256);
    let token = encode(
        &headers,
        &user_claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}
