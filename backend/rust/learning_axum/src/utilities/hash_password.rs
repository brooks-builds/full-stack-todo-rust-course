use dotenvy_macro::dotenv;
use eyre::Result;

pub fn hash_password(password: &str) -> Result<String> {
    let cost: &str = dotenv!("HASH_COST");
    let hash = bcrypt::hash(password, cost.parse()?)?;
    Ok(hash)
}
