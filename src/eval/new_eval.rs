use crate::commands::new::{NewCommands, SubNewCommands};
use crate::functions::{module, shell, pkgs};
use crate::types;


pub fn new_eval(commands: NewCommands ) {
    match commands {
        NewCommands::Shell { shell_type, name, nixpkgs, unfree, package, pkgs, env, overlays  } => {
            if let Some(shell) = shell_type {
                match  shell {
                    types::ShellType::Rust => shell::rust_shell(
                        name,
                        nixpkgs,
                        unfree,
                        package,
                        pkgs,
                        env,
                        overlays
                    ).unwrap(),

                }; 
            } else {
                shell::default_shell(
                    name,
                    nixpkgs,
                    unfree,
                    package,
                    pkgs,
                    env,
                    overlays
                ).unwrap()
            }
        },
    }
}

pub fn sub_new_eval(commands: SubNewCommands, command_type: bool ) {
    match commands {
       SubNewCommands::Module  {  name, import, outer_import, pkgs } => 
            module::default_module(
                name,
                command_type,
                import,
                outer_import,
                pkgs
            ).unwrap(),

        SubNewCommands::Pkgs => 
            pkgs::pkgs(
                command_type
            ).unwrap(),

    }
}
