# import niv sources and the pinned nixpkgs
{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
let
  # import rust compiler
  rust = import ./nix/rust.nix { inherit sources; };

  # configure naersk to use our pinned rust compiler
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };

  # tell nix-build to ignore the `target` directory
  mkSrc = dir:
    builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    dir;

  mkRustPackage = dir:
    naersk.buildPackage {
      src = mkSrc dir;
      remapPathPrefix = true;
    };
in {
  guessing-game = mkRustPackage ./guessing-game;
  helloworld = mkRustPackage ./helloworld;
}
