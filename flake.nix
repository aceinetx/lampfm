{
  description = "lampfm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
            rust-overlay.overlays.default
          ];
        };
        lib = pkgs.lib;

        stableToolchain = fenix.packages.${system}.complete.toolchain;
        rustAnalyzer = fenix.packages.${system}.latest.rust-analyzer;
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            wayland-protocols
            wayland
            libxkbcommon
            libGL
          ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            lib.flatten [
              stableToolchain
              rustAnalyzer
              cargo-expand

              u-config
              wayland
              wayland-protocols
            ];

          shellHook = ''
            export LD_LIBRARY_PATH="${libPath}"
          '';
        };

        packages.default =
          let
            wrapped = pkgs.rustPlatform.buildRustPackage {
              pname = "lampfm";
              version = "1.0.0";

              src = self;

              cargoLock.lockFile = ./Cargo.lock;

              nativeBuildInputs =
                with pkgs;
                lib.flatten [
                  stableToolchain
                  rustAnalyzer
                  cargo-expand

                  u-config
                  wayland
                  wayland-protocols
                ];
            };
          in
          pkgs.symlinkJoin {
            name = "lampfm-wrapped";
            paths = [ wrapped ];
            buildInputs = [ pkgs.makeWrapper ];
            postBuild = ''
              mv "$out/bin/lampfm" "$out/bin/lampfm-real"

              makeWrapper "$out/bin/lampfm-real" "$out/bin/lampfm" \
              	--set LD_LIBRARY_PATH "${libPath}"
            '';
          };
      }
    );
}
