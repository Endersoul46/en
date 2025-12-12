use crate::commands::new::{NewCommands, SubNewCommands};
use clap::{Subcommand};

#[derive(Subcommand, Debug)]
pub enum BaseCommands {
    Home {
        #[command(subcommand)]
        commands: SubBaseCommands
    }, 

    NixOS {
        #[command(subcommand)]
        commands: SubBaseCommands
    },

    Update,

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

    Update
}


