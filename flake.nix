{
  description = "objset";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-21.11";
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
  }: let
    emptyOverlay = final: prev: {};
    objset-drv = pkgs:
      pkgs.rustPlatform.buildRustPackage {
        pname = "objset";
        version = "v0.1.0";

        src = ./.;

        buildFeatures = [ "pyo3" ];

        cargoLock = {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
        };
      };
    objset-python-drv = pkgs: isNew: pythonPackages:
      pythonPackages.buildPythonPackage rec {
        pname = "objset";
        version = "v0.1.0";

        src = ./.;

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
        };

        format = "pyproject";

        # HACK: maturinBuildHook is dumb and doesn't read pyproject.toml for some reason
        maturinBuildFlags = if isNew then ["--all-features"] else [''--cargo-extra-args="--all-features"''];

        nativeBuildInputs = with pkgs.rustPlatform; [cargoSetupHook maturinBuildHook];

        # needed for maturin
        propagatedBuildInputs = with pythonPackages; [cffi];
      };
    pythonOverride = prev: isNew: (prevArgs: {
      packageOverrides = let
        ourOverlay = new: old: {
          objset = objset-python-drv prev isNew old;
        };
      in
        prev.lib.composeExtensions
        prevArgs.packageOverrides or emptyOverlay
        ourOverlay;
    });
  in
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
      in rec {
        packages = rec {
          objset = objset-drv pkgs;
          objset-python = objset-python-drv pkgs false pkgs.python3Packages;
          default = objset;
        };
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "cargo" "rustc"];
            })
            gcc
          ];

          RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
            extensions = ["rust-src"];
          }}/lib/rustlib/src/rust/library";
          buildInputs = with pkgs; [
            maturin
            rust-analyzer
            (pkgs.python3.withPackages (p:
              with p; [
                cffi
              ]))
          ];
        };
        devShells.python = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            (pkgs.python3.withPackages (p:
              with p; [
                packages.objset-python
              ]))
          ];
        };
      }
    )
    // {
      overlays.default = final: prev: rec {
        objset = objset-drv prev;
        python3 = prev.python3.override (pythonOverride prev true);
        python310 = prev.python310.override (pythonOverride prev true);
        python39 = prev.python39.override (pythonOverride prev false);
      };
    };
}
