{
  description = "objset";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
      in {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.nightly.latest.default.override {
              extensions = ["rust-src" "cargo" "rustc"];
            })
            gcc
          ];

          RUST_SRC_PATH = "${pkgs.rust-bin.nightly.latest.default.override {
            extensions = ["rust-src"];
          }}/lib/rustlib/src/rust/library";
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
