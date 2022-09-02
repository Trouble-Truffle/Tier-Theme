{
  description = "A Tierlist helper";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inputs = [
          rust
          pkgs.pkg-config
          pkgs.glib
          pkgs.gdk-pixbuf
          pkgs.pango
          pkgs.cairo
          pkgs.graphene
          pkgs.gtk4
        ];

        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

      in
      {
        defaultPackage = rustPlatform.buildRustPackage {
          pname = "Tiertheme";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = inputs;
          buildPhase = ''
            echo "Building..."
          '';
          installPhase = "echo 'Skipping installPhase'";
        };

        devShell = pkgs.mkShell { packages = inputs; };
      });
}
