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
  ch2-guessing-game = mkRustPackage ./ch2/guessing-game;
  ch2-helloworld = mkRustPackage ./ch2/helloworld;
  ch3-branches = mkRustPackage ./ch3/branches;
  ch3-functions = mkRustPackage ./ch3/functions;
  ch3-fibonacci = mkRustPackage ./ch3/fibonacci;
  ch3-loops = mkRustPackage ./ch3/loops;
  ch3-variables = mkRustPackage ./ch3/variables;
  ch4 = mkRustPackage ./ch4;
  ch5-examples = mkRustPackage ./ch5/examples;
  ch5-rectangles = mkRustPackage ./ch5/rectangles;
  ch7-communicator = mkRustPackage ./ch7/communicator;
  ch7-privacy = mkRustPackage ./ch7/privacy;
}
