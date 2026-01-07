use crate::commands::new::{NewCommands, SubNewCommands};
use crate::functions::{pkgs,update};
use clap::{Subcommand};

#[derive(Subcommand, Debug)]
pub enum BaseCommands {
    Home {
        #[command(subcommand)]
        commands: SubBaseCommands
    }, 

    Nixos {
        #[command(subcommand)]
        commands: SubBaseCommands
    },

    Update,

    Host,

    Theme,

    New {
        #[command(subcommand)]
        commands: NewCommands
    }, 




}

#[derive(Subcommand, Debug)]
pub enum SubBaseCommands {
    New {
        #[command(subcommand)]
        commands: SubNewCommands
    }, 

    Pkgs,

    Update
}

impl SubBaseCommands {
    pub fn eval (self, command_type: bool) {

        match self {
            SubBaseCommands::New { commands } => commands.eval(command_type),
            SubBaseCommands::Pkgs => 
                pkgs::pkgs(
                    command_type
                ).unwrap(),

            SubBaseCommands::Update => {
                if command_type {
                    match update::nix_os_update() {
                        Ok(_) => {},
                        Err(_) => {
                            println!("update failed, error:");
                        }
                    }
                }else{
                    match update::home_update(){
                        Ok(_) => {},
                        Err(_) => {
                            println!("update failed, error:");
                        }
                    }
                }
            }
        }

    }

}

