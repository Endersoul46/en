self: super: {
      en = ./default.nix  {
        inherit (super) lib rustPlatform fetchFromGitHub;
      };
      }
