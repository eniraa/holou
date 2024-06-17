{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        binBuild = rustPlatform.buildRustPackage {
          pname = "holou";
          version = "0.1.0";
          src = ./.;

          buildInputs = [
            pkgs.llvm 
            pkgs.clang 
            pkgs.libclang 

            # apple bullshit
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration 
          ];

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "qhull-0.3.1" = "sha256-E8Xbq6Cb2ATyLDIw5hBg7dtyYCFFgV+KBuW8vILHw98=";
            };
          };
        };
      in {
        packages = {
          rustPackage = binBuild;
        };
        defaultPackage = binBuild;
        devShell = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override { extensions = [ "rust-src" ]; }) 
            pkgs.llvm 
            pkgs.clang 
            pkgs.libclang

            # apple bullshit
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration 
          ];
        };
      }
    );
}
