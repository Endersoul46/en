{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "en";
  version = "unstable-2026-01-08";

  src = fetchFromGitHub {
    owner = "Pavedd";
    repo = "en";
    rev = "8535ea23283e66f7748354d9de515c78edc23fc0";
    hash = "sha256-AwcbQTbCkZJYD+7p79v2mIvE5DOB1RLjLutSVnvMNh4=";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "A rust cli Helper untility for my NixOS config";
    homepage = "https://github.com/Pavedd/en";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ ];
    mainProgram = "en";
  };
}
