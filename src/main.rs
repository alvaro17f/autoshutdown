mod modules;
mod utils;

use crate::utils::{completions::set_completions, generate_config::generate_config};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use color_print::cprintln;
use modules::{
    commands::{get_status, set_status},
    zenity::zenity,
};
use std::process::exit;
use utils::{completions::print_completions, get_config::get_config};

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // If provided, generate completions for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    /// List of available commands
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Enable autoshutdown
    Enable,
    /// Disable autoshutdown
    Disable,
    /// Show current status
    Status,
}

fn main() -> Result<()> {
    generate_config()?;
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        if generator == Shell::Zsh || generator == Shell::Bash {
            set_completions(generator, &mut cmd);
            cprintln!("<c>{}</c> <y>completions are set", generator);
            exit(0)
        } else {
            print_completions(generator, &mut cmd);
            exit(0)
        }
    }
    match &cli.commands {
        Some(Commands::Enable) => {
            set_status(true)?;
        }
        Some(Commands::Disable) => {
            set_status(false)?;
        }
        Some(Commands::Status) => {
            get_status()?;
        }
        None => {
            let status = get_config()?;
            if status {
                zenity()?
            }
        }
    }

    Ok(())
}
