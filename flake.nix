{
  description = "CHEESE";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    #fokquote.url = "github:fokohetman/fokquote";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      #config = import ./configuration.nix;
      lib = nixpkgs.lib;
    in {
      formatter = pkgs.alejandra;
      packages.default =
        pkgs.runCommand "chess" {
          buildInputs = [pkgs.rustc pkgs.gcc];
          src = ./src;
          /*quotes =
            ["["]
            ++ (lib.lists.forEach config.quotes (x: "[\"" + toString (builtins.elemAt x 0) + "\" \"" + toString (builtins.elemAt x 1) + "\"]"))
            ++ ["]"];
          plush =
            ["["]
            ++ (lib.lists.forEach config.plush (x: "\"" + toString x + "\""))
            ++ ["]"];
          #quotes = "[[\"test quote\" \"fokfok\"]]";*/
        } ''
          #export "CONFIG={quotes=$quotes; plush=$plush}" #;$plush"
          mkdir -p "$out/bin"
          rustc "$src/chess.rs" -o "$out/bin/chess";

        '';
    });
}
