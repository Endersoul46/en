use clap::{Parser};
use en::commands::{new::NewCommands, base::BaseCommands};
use en::eval::new_eval::new_eval;
use en::functions::{ module, shell, update, pkgs};
use en::eval::{self, base_eval, new_eval};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: BaseCommands
}

 






fn main() {
    let cli = Cli::parse();
    match cli.commands {
            BaseCommands::NixOS { commands } => {
                base_eval::sub_base_eval(commands, true);
            },
            BaseCommands::Home { commands } => {
                base_eval::sub_base_eval(commands, false);
            },
 
            BaseCommands::Update => update::update().expect("update failed"), 

            BaseCommands::New { commands } => new_eval(commands),
        }
    }
