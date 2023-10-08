use anyhow::Result;
use std::env;

pub fn get_user() -> Result<String> {
    Ok(env::var("USER").expect("USER environment variable not set"))
}
