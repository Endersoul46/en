{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "en";
  version = "unstable-2025-11-30";

  src = fetchFromGitHub {
    owner = "Endersoul46";
    repo = "en";
    rev = "d0d811bbda5954d07c32a04aa2d6222d8b28b68b";
    hash = "sha256-dA7OMDGRdwC0VV59luxpB3sWFU+S+0TSVHB0bmXeUFM=";
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

  meta = {
    description = "A rust cli Helper untility for my NixOS config";
    homepage = "https://github.com/Endersoul46/en";
    license = lib.licenses.unfree; # FIXME: nix-init did not find a license
    maintainers = with lib.maintainers; [ ];
    mainProgram = "en";
  };
}
