{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "en";
  version = "unstable-2025-12-12";

  src = fetchFromGitHub {
    owner = "Endersoul46";
    repo = "en";
    rev = "893ee07562e3d457225cc56602803e58ceace31e";
    hash = "sha256-K1PGhSnuct3Kt7tyH30MwIzX7BpNwJQp9maDkRCBJ60=";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "A rust cli Helper untility for my NixOS config";
    homepage = "https://github.com/Endersoul46/en";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ ];
    mainProgram = "en";
  };
}
