{
  description = "X11 window manager";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, nixpkgs, naersk, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.default.toolchain;
        naerskLib = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
        nativeBuildInputs = with pkgs; [
          openssl
          pkg-config
          glib
          cairo
          pango
        ];
        buildInputs = with pkgs; [
          toolchain
          python3
        ];
      in with pkgs;
      {
        packages = rec {
          mywm = naerskLib.buildPackage {
            name = "mywm";
            src = ./.;
            inherit buildInputs;
            inherit nativeBuildInputs;
          };
          default = mywm;
        };
        overlays = {
          mywm = buildPkg packages.mywm;
        };
        devShells.default = mkShell {
          inherit buildInputs;
          inherit nativeBuildInputs;
        };
      });
}
