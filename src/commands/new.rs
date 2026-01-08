use clap::{Subcommand};
use crate::types;
use crate::functions::{module,shell};


#[derive(Subcommand, Debug)]
pub enum NewCommands {
    Shell {
        #[arg(default_value = None )]
        shell_type: Option<types::ShellType>,

        #[arg(long, short, default_value = "default")]
        name: String,

        #[arg(long, default_value = "github:nixos/nixpkgs/nixos-25.11")]
        nixpkgs: Option<String>,

        #[arg(long, default_value = "3.14")]
        python_version: Option<String>,


        #[arg(long,short , default_value_t = true)]
        unfree: bool,

        #[arg(long , default_value_t = false)]
        package: bool,



        #[arg(
            long,
            short,
            num_args = 1..,
            value_delimiter = ' ',
            default_missing_values = &[""]
        )]
        pkgs: Option<Vec<String>>,

        #[arg(
            long,
            short,
            num_args = 0..,
            value_delimiter = ' ',
            default_missing_values = &[""]
        )]
        env: Option<Vec<String>>,

        #[arg(
            long,
            short,
            num_args = 0..,
            value_delimiter = ' ',
            default_missing_values = &[""]
        )]
        overlays: Option<Vec<String>>,





    },
}


impl NewCommands {
    pub fn eval(self) {
        match self {
            NewCommands::Shell { shell_type, name, nixpkgs, python_version, unfree, package, pkgs, env, overlays  } => {
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

                        types::ShellType::Python => shell::python_shell(
                            name,
                            nixpkgs,
                            python_version,
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


}

 #[derive(Subcommand, Debug)]
pub enum SubNewCommands {
   Module {
        #[arg(long, short, default_value = "default")]
        name: String,

        #[arg(
            long,
            short,
            num_args = 1..,
            value_delimiter = ' ',
            default_values_t = vec!["pkgs".to_string(), "config".to_string()]
        )]
        import: Vec<String>,

        #[arg(
            long,
            short,
            num_args = 0..,
            value_delimiter = ' ',
            default_missing_values = &["import"]
        )]
        outer_import: Option<Vec<String>>,

        #[arg(
            long,
            short,
            num_args = 0..,
            value_delimiter = ' ',
            default_missing_values = &[""]
        )]
        pkgs: Option<Vec<String>>,


    },

}

impl SubNewCommands {
    pub fn eval(self, command_type: bool ) {
        match self {
            SubNewCommands::Module  {  name, import, outer_import, pkgs } => 
                module::default_module(
                    name,
                    command_type,
                    import,
                    outer_import,
                    pkgs
                ).unwrap(),

        }
    }

}


