use crate::utils::get_user::get_user;
use anyhow::Result;
use color_print::{cformat, cprintln};
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

pub fn set_status(status: bool) -> Result<()> {
    let user = get_user()?;
    let file_path = format!("/home/{user}/.config/autoshutdown/autoshutdown.toml");

    // Open the file for reading and writing
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_contents = if status {
        "ENABLED = true".to_string()
    } else {
        "ENABLED = false".to_string()
    };

    // Seek back to the beginning of the file and truncate it
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    // Write the new contents to the file
    file.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn get_status() -> Result<()> {
    // read the content of the autoshutdown.toml file and print the status
    let user = get_user()?;
    let file_path = format!("/home/{user}/.config/autoshutdown/autoshutdown.toml");

    // Open the file for reading and writing
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let status = contents
        .split('=')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .trim()
        .to_string();
    let status = if status == "true" {
        cformat!("<g>ENABLED")
    } else {
        cformat!("<r>DISABLED")
    };

    cprintln!("<c>Current status: </><s>{status}");
    Ok(())
}
