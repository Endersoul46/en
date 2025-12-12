use std::env;
use std::io::{self};
use duct::cmd;


pub fn pkgs(pkg_type: bool) -> Result<(), io::Error> {
    let editor = env::var("EDITOR").expect("Editor wansn't found");
   let mut pkg_file = env::var("PKGS_FILE").expect("Pkgs file wansn't found");
   if !pkg_type{
       pkg_file = env::var("PKGS_HOME_FILE").expect("Pkgs file wansn't found");
   }

        
    cmd!(editor, pkg_file).run()?;
    Ok(())
}
