use clap::{Parser, Subcommand};
use en::{types::{self}};
use en::commands::{new::NewCommands };
use en::functions::{ module, shell, update};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: BaseCommands
}

#[derive(Subcommand, Debug)]
enum BaseCommands {
    New {
        #[command(subcommand)]
        commands: NewCommands
    },

    Update

} 






fn main() {
    let cli = Cli::parse();
    match cli.commands {
        BaseCommands::New { commands } => {
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
                NewCommands::Module  { name, module_type, import, outer_import, pkgs } => 
                    module::default_module(
                        name,
                        module_type,
                        import,
                        outer_import,
                        pkgs
                    ).unwrap(),
            }
        },
        BaseCommands::Update => update::update().expect("update failed"), 
    };
}
