{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "en";
  version = "unstable-2025-12-03";

  src = fetchFromGitHub {
    owner = "Endersoul46";
    repo = "en";
    rev = "b868db0c30bae42ebf253d4e42ec8d63bbf9ef90";
    hash = "sha256-D2dNKGAkwS7bs7ZQLdHjxu/+v746lEzpPWr9oThnU/o=";
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
