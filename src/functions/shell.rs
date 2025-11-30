
use std::io::{self, Write, BufWriter};
use std::fs::OpenOptions;

fn devenv_init() -> io::Result<()>{
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(".envrc")?;
    writeln!(file, "use flake 2>&1 | nom")?;
    Ok(())
}

pub fn default_shell(
    name: String,
    nixpkgs: Option<String>,
    unfree: bool,
    pkgs: Option<Vec<String>>,
    env: Option<Vec<String>>,
    overlays: Option<Vec<String>>,
) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("flake.nix")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{{")?;
    writeln!(
        writer,
        "  description = \"A Nix-flake development environment for {}\";\n",
        name
    )?;

    writeln!(writer, "  inputs = {{")?;
    writeln!(
        writer,
        "    nixpkgs.url = \"{}\";",
        nixpkgs.expect("pkgs unwrap failed")
    )?;
    writeln!(writer, "  }};\n")?;

    writeln!(writer, "  outputs =")?;
    writeln!(writer, "    {{ self, ... }}@inputs:\n")?;

    writeln!(writer, "    let")?;
    writeln!(writer, "      supportedSystems = [")?;
    writeln!(writer, "        \"x86_64-linux\"")?;
    writeln!(writer, "        \"aarch64-linux\"")?;
    writeln!(writer, "        \"x86_64-darwin\"")?;
    writeln!(writer, "        \"aarch64-darwin\"")?;
    writeln!(writer, "      ];")?;

    writeln!(writer, "      forEachSupportedSystem =")?;
    writeln!(writer, "        f:")?;
    writeln!(
        writer,
        "        inputs.nixpkgs.lib.genAttrs supportedSystems ("
    )?;
    writeln!(writer, "          system:")?;
    writeln!(writer, "          f {{")?;
    writeln!(writer, "            pkgs = import inputs.nixpkgs {{")?;
    writeln!(writer, "              inherit system;")?;

    if unfree {
        writeln!(writer,"              config.allowUnfree = true;")?;
    };

    if let Some(over) = overlays {
        writeln!(writer, "              overlays = [")?;
        for overlay in over {
            writeln!(writer, "                {}", overlay)?;
        }
        writeln!(writer, "              ];")?;
    }

    writeln!(writer, "            }};")?;
    writeln!(writer, "          }}")?;
    writeln!(writer, "        );")?;
    writeln!(writer, "    in")?;

    writeln!(writer, "    {{")?;

    writeln!(writer, "      devShells = forEachSupportedSystem (")?;
    writeln!(writer, "        {{ pkgs }}:")?;
    writeln!(writer, "        {{")?;
    writeln!(writer, "          default = pkgs.mkShell {{")?;

    if let Some(packet) = pkgs {
        writeln!(writer, "            packages = with pkgs; [")?;
        for pkg in packet {
            writeln!(writer, "              {}", pkg)?;
        }
        writeln!(writer, "            ];")?;
    }

    if let Some(environment) = env {
        writeln!(writer, "            env = {{")?;
        for val in environment {
            writeln!(writer, "              {};", val)?;
        }
        writeln!(writer, "            }};")?;
    }

    writeln!(writer, "          }};")?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "      );")?;
    writeln!(writer, "    }};")?;
    writeln!(writer, "}}")?;

    devenv_init()?;

    Ok(())
}

pub fn rust_shell(
    name: String,
    nixpkgs: Option<String>,
    unfree: bool,
    pkgs: Option<Vec<String>>,
    env: Option<Vec<String>>,
    overlays: Option<Vec<String>>,
) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("flake.nix")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{{")?;
    writeln!(
        writer,
        "  description = \"A Nix-flake-based Rust development environment for{}\";\n",
        name
    )?;

    writeln!(writer, "  inputs = {{")?;
    writeln!(
        writer,
        "    nixpkgs.url = \"{}\";",
        nixpkgs.expect("pkgs unwrap failed")
    )?;
    writeln!(writer, "    fenix = {{")?;
    writeln!(
        writer,
        "      url = \"github:nix-community/fenix/monthly\";"
    )?;
    writeln!(writer, "      inputs.nixpkgs.follows = \"nixpkgs\";")?;
    writeln!(writer, "    }};")?;
    writeln!(writer, "  }};\n")?;

    writeln!(writer, "  outputs =")?;
    writeln!(writer, "    {{ self, ... }}@inputs:\n")?;

    writeln!(writer, "    let")?;
    writeln!(writer, "      supportedSystems = [")?;
    writeln!(writer, "        \"x86_64-linux\"")?;
    writeln!(writer, "        \"aarch64-linux\"")?;
    writeln!(writer, "        \"x86_64-darwin\"")?;
    writeln!(writer, "        \"aarch64-darwin\"")?;
    writeln!(writer, "      ];")?;

    writeln!(writer, "      forEachSupportedSystem =")?;
    writeln!(writer, "        f:")?;
    writeln!(
        writer,
        "        inputs.nixpkgs.lib.genAttrs supportedSystems ("
    )?;
    writeln!(writer, "          system:")?;
    writeln!(writer, "          f {{")?;
    writeln!(writer, "            pkgs = import inputs.nixpkgs {{")?;
    writeln!(writer, "              inherit system;")?;
    if unfree {
        writeln!(writer,"              config.allowUnfree = true;")?;
    };

    writeln!(writer, "              overlays = [")?;
    writeln!(writer, "                inputs.self.overlays.default")?;

    if let Some(over) = overlays {
        for overlay in over {
            writeln!(writer, "                {}", overlay)?;
        }
    }

    writeln!(writer, "              ];")?;
    writeln!(writer, "            }};")?;
    writeln!(writer, "          }}")?;
    writeln!(writer, "        );")?;
    writeln!(writer, "    in")?;

    writeln!(writer, "    {{")?;

    writeln!(writer, "      overlays.default = final: prev: {{")?;
    writeln!(writer, "        rustToolchain =")?;
    writeln!(
        writer,
        "          with inputs.fenix.packages.${{prev.stdenv.hostPlatform.system}};"
    )?;
    writeln!(writer, "          combine (")?;
    writeln!(writer, "            with stable;")?;
    writeln!(writer, "            [")?;
    writeln!(writer, "              clippy")?;
    writeln!(writer, "              rustc")?;
    writeln!(writer, "              cargo")?;
    writeln!(writer, "              rustfmt")?;
    writeln!(writer, "              rust-src")?;
    writeln!(writer, "            ]")?;
    writeln!(writer, "          );")?;
    writeln!(writer, "      }};\n")?;

    writeln!(writer, "      devShells = forEachSupportedSystem (")?;
    writeln!(writer, "        {{ pkgs }}:")?;
    writeln!(writer, "        {{")?;
    writeln!(writer, "          default = pkgs.mkShell {{")?;

    writeln!(writer, "            packages = with pkgs; [")?;
    writeln!(writer, "              rustToolchain")?;
    writeln!(writer, "              openssl")?;
    writeln!(writer, "              pkg-config")?;
    writeln!(writer, "              cargo-deny")?;
    writeln!(writer, "              cargo-edit")?;
    writeln!(writer, "              cargo-watch")?;
    writeln!(writer, "              rust-analyzer")?;

    if let Some(packet) = pkgs {
        for pkg in packet {
            writeln!(writer, "              {}", pkg)?;
        }
    }

    writeln!(writer, "            ];")?;

    writeln!(writer, "            env = {{")?;
    writeln!(
        writer,
        "              # Required by rust-analyzer\n              RUST_SRC_PATH = \"${{pkgs.rustToolchain}}/lib/rustlib/src/rust/library\";"
    )?;

    if let Some(environment) = env {
        for val in environment {
            writeln!(writer, "              {};", val)?;
        }
    }

    writeln!(writer, "            }};")?;
    writeln!(writer, "          }};")?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "      );")?;
    writeln!(writer, "    }};")?;
    writeln!(writer, "}}")?;

    devenv_init()?;

    Ok(())
}

