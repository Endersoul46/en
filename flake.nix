{
  description = "A Nix-flake-based Rust development environment fordefault";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              config.allowUnfree = true;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      packages = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.callPackage ./default.nix {};
      });
      overlay = import ./en-overlay.nix;
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              openssl
              pkg-config
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer
            ];
            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );
    };
}
