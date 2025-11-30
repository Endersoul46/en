use crate::types;
use clap::{Subcommand};

 #[derive(Subcommand, Debug)]
pub enum NewCommands {
    Shell {
        #[arg(default_value = None )]
        shell_type: Option<types::ShellType>,

        #[arg(long, short, default_value = "default")]
        name: String,

        #[arg(long, default_value = "github:nixos/nixpkgs/nixos-25.05")]
        nixpkgs: Option<String>,

        #[arg(long,short , default_value_t = true)]
        unfree: bool,

        #[arg(long , default_value_t = true)]
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
    Module {
        #[arg(long, short, default_value = "default")]
        name: String,

        module_type: types::ModuleType,

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
