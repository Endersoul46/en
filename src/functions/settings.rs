use std::env;
use std::io::{self};
use duct::cmd;


pub fn change_hostname() -> Result<(), io::Error> {
    let editor = env::var("EDITOR").expect("Editor wansn't found");
    let home_manager_file = env::var("HOSTNAME_FILE").expect("Honstname file wansn't found");
        
    cmd!(editor, home_manager_file).run()?;
    Ok(())
}

pub fn change_theme() -> Result<(), io::Error> {
    let editor = env::var("EDITOR").expect("Editor wansn't found");
    let theme_file = env::var("THEME_FILE").expect("theme file wansn't found");
        
    cmd!(editor, theme_file).run()?;
    Ok(())
}
