{ import, ... }: 
  flake.nixosModules.cool = { pkgs, config, ... }: 
    environment.systemPackages = with pkgs; [
      hi
    ]
  };
}
