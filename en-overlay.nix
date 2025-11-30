self: super: {
      en = import ./default.nix  {
        inherit (super) lib rustPlatform fetchFromGitHub;
      };
      }
