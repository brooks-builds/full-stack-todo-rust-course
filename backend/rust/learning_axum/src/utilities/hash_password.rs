use dotenvy_macro::dotenv;
use eyre::Result;

pub fn hash_password(password: &str) -> Result<String> {
    let cost: &str = dotenv!("HASH_COST");
    let hash = bcrypt::hash(password, cost.parse()?)?;
    Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
    let is_verified = bcrypt::verify(password, hash)?;
    Ok(is_verified)
}
