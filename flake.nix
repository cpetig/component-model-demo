{
  outputs = {
    flake-parts,
    crane,
    fenix,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"];
      perSystem = {
        config,
        system,
        lib,
        pkgs,
        ...
      }: let
        toolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            complete.rust-src
            complete.rustfmt
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
        craneLib = crane.lib.${system}.overrideToolchain toolchain;
      in {
        packages.default = craneLib.buildPackage {
          src = ./.;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [config.packages.default];
          packages = with fenix.packages.${system} // pkgs; [
            rust-analyzer
            wasmtime
            rustfmt
            gcc
          ];
        };
      };
    };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
