{
  description = "A basic flake with a shell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) (final: prev: {
              nodejs = prev.nodejs-14_x;
        }) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            yarn
            nodejs-14_x
            openssl
            pkg-config
            exa
            fd
            cargo
            rust
            rustfmt
            wasm-pack
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
        };
      }
    );
}
