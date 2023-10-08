use crate::utils::get_user::get_user;
use anyhow::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

pub fn generate_config() -> Result<()> {
    let user = get_user()?;
    let config_path: PathBuf =
        format!("/home/{user}/.config/autoshutdown/autoshutdown.toml").into();
    if !config_path.exists() {
        create_dir_all(config_path.parent().unwrap())?;
        let mut file = File::create(config_path)?;
        let new_contents = "ENABLED = true".to_string();
        file.write_all(new_contents.as_bytes())?;
    }
    Ok(())
}
