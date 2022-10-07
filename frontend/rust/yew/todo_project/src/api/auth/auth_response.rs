use serde::*;

use super::auth::Auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub data: Auth,
}
