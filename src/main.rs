use clap::{Parser};
use en::commands::base::BaseCommands;
use en::functions::{settings,update};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: BaseCommands
}

 






fn main() {
    let cli = Cli::parse();
    match cli.commands {
            BaseCommands::Nixos { commands } => {
                commands.eval(true);
            },
            BaseCommands::Home { commands } => {
                commands.eval(false);
            },
 
            BaseCommands::Update => update::update().expect("update failed"),

            BaseCommands::Host => settings::change_hostname().expect("Hostname file not found"),

            BaseCommands::Theme => settings::change_theme().expect("Theme file not found"),

            BaseCommands::New { commands } => commands.eval(),
        }
    }
