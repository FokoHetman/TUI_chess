{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, nixpkgs }:
  flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = nixpkgs.legacyPackages.${system};
        config = import ./configuration.nix;
      in
      {
        formatter = pkgs.alejandra;
        packages.default = let 
          mf = (pkgs.lib.importTOML ./Cargo.toml).package;
        in
          pkgs.rustPlatform.buildRustPackage rec {
            pname = mf.name;
            version = mf.version;
            src = pkgs.lib.cleanSource ./.;

            cargoLock.lockFile = ./Cargo.lock;
            
            CONFIG = "{}";  # not used, somewhere int eh futurr

            cargoHash = "sha256-SkFGStZShqocYwzyU7ylaQZ2+YRmHNCUqkCAvwFt1+c=";#nixpkgs.lib.fakeHash;
            #cargoSha256 = nixpkgs.lib.fakeSha256;
          };
      });
}
