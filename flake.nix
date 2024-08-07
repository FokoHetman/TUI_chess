{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        defaultPackage = (pkgs.makeRustPlatform {
          inherit (fenix.packages.${system}.minimal) cargo rustc;
        }).buildRustPackage {
          pname = "chess";
          version = "0.1.0";
          src = ./src;
          cargoHash = "sha256-SkFGStZShqocYwzyU7ylaQZ2+YRmHNCUqkCAvwFt1+c=";#nixpkgs.lib.fakeHash;
          #cargoSha256 = nixpkgs.lib.fakeSha256;
        };
      });
}
