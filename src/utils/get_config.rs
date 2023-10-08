use crate::utils::{get_user::get_user, macros::error};
use anyhow::Result;
use config::Config;
use std::collections::HashMap;

pub fn get_config() -> Result<bool> {
    let user = get_user()?;

    let settings = Config::builder()
        .add_source(config::File::with_name(&format!(
            "/home/{user}/.config/autoshutdown/autoshutdown.toml"
        )))
        .build()?;
    let config = settings.try_deserialize::<HashMap<String, String>>()?;
    let status = config
        .get("ENABLED")
        .ok_or(error!("ENABLED key not found"))?
        .to_owned()
        .to_string();

    if status == "true" {
        Ok(true)
    } else {
        Ok(false)
    }
}
