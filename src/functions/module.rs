use std::io::{self, Write, BufWriter};
use std::fs::{OpenOptions};
use crate::types;

pub fn default_module (
    name: String,
    module_type: bool,
    import: Vec<String>,
    outer_import: Option<Vec<String>>,
    pkgs: Option<Vec<String>>,
) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{}.nix", name))?;

    let mut writer = BufWriter::new(file);

    if let Some(import) = outer_import {
        write!(writer, "{{ ")?;
        for i in import {
            write!(writer, "{}, ", i)?;
        };
        writeln!(writer, "... }}: ")?;

    }else{
        writeln!(writer, "{{")?;
    };

  //checks if home or nixos module 
  if module_type {
      write!(writer, "  flake.nixosModules.{} = {{ ", name)?;
  }else{
      write!(writer, "  flake.homeModules.{} = {{" , name)?;
  }
    for i in import {
        write!(writer, "{}, ", i)?;
    };
    writeln!(writer, "... }}: ")?;

    if let Some(packets) = pkgs {
        writeln!(writer, "    environment.systemPackages = with pkgs; [")?;
        for pkg in packets {
            writeln!(writer, "      {}", pkg)?;
        };
        writeln!(writer, "    ]")?;

    };

    writeln!(writer, "  }};")?;
    writeln!(writer, "}}")?;
    Ok(())
}
