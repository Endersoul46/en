use clap::{ ValueEnum};


#[derive(ValueEnum,Clone, Debug)]
pub enum ModuleType {
    Home,
    NixOS,
}

#[derive(ValueEnum,Clone, Debug)]
pub enum ShellType {
    Rust,
    Python,
}
