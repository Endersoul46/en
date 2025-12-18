{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "en";
  version = "unstable-2025-12-18";

  src = fetchFromGitHub {
    owner = "Endersoul46";
    repo = "en";
    rev = "2dcf0eb8792462baa3cec55b012a071aa5e61c18";
    hash = "sha256-lxml0ufoU8nKTQQ6Q8R4Hk87vaKKEPmnukNpiQMnTCA=";
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
