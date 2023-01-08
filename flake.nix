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
    objset-drv = pkgs: override:
      pkgs.rustPlatform.buildRustPackage {
        pname = "objset";
        version = "v0.1.0";

        src = ./.;

        cargoLock = {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
        };
      } // override;
    objset-python-drv = pkgs: pythonPackages:
      pythonPackages.buildPythonPackage rec {
        pname = "objset";
        version = "v0.1.0";

        src = ./.;

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
        };

        format = "pyproject";

        nativeBuildInputs = with pkgs.rustPlatform; [cargoSetupHook maturinBuildHook];

        # needed for maturin
        propagatedBuildInputs = with pythonPackages; [cffi];
      };
    pythonOverride = prev: (prevArgs: {
      packageOverrides = let
        ourOverlay = new: old: {
          objset = objset-python-drv prev old;
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
        pkgs-cross-mingw = import nixpkgs {
          inherit system;
          crossSystem = {
              config = "x86_64-w64-mingw32";
            };
        };
        mingw_w64_cc = pkgs-cross-mingw.stdenv.cc;
        mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
        mingw_w64_pthreads_w_static = pkgs-cross-mingw.windows.mingw_w64_pthreads.overrideAttrs (oldAttrs: {
          # TODO: Remove once / if changed successfully upstreamed.
          configureFlags = (oldAttrs.configureFlags or []) ++ [
            # Rustc require 'libpthread.a' when targeting 'x86_64-pc-windows-gnu'.
            # Enabling this makes it work out of the box instead of failing.
            "--enable-static"
          ];
        });
      in rec {
        packages = rec {
          objset = objset-drv pkgs {};
          objset-w64-gnu = pkgs-cross-mingw.rustPlatform.buildRustPackage {
            pname = "objset-w64-gnu";
            version = "v0.1.0";

            src = ./.;

            cargoLock = {
              # Why I yes, I would like not writing the hash of my Cargo.lock very much.
              lockFile = ./Cargo.lock;
            };
            target = "x86_64-pc-windows-gnu";
            doCheck=false;

            # buildInputs = with pkgs; [
            #   pkgsCross.mingwW64.stdenv.cc
            #   pkgsCross.mingwW64.buildPackages.binutils
            #   pkgsCross.mingwW64.windows.pthreads
            # ];
          };
          objset-python = objset-python-drv pkgs pkgs.python3Packages;
          default = objset;
        };
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "cargo" "rustc"];
            })
            gcc
          ];

          # RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
          #   extensions = ["rust-src"];
          # }}/lib/rustlib/src/rust/library";
          # PYO3_MINGW_DLLTOOL="${pkgs.llvmPackages.bintools-unwrapped}/bin/llvm-dlltool";
          buildInputs = with pkgs; [
            maturin
            rust-analyzer
            # for targetting x86_64-pc-windows-gnu
            # (binutils.override { withAllTargets=true; })
            pkgsCross.mingwW64.buildPackages.binutils
            # pkgsCross.mingwW64.stdenv.cc
            mingw_w64_cc
            # binutils-unwrapped-all-targets
            # for targetting x86_64-pc-windows-msvc
            llvmPackages.bintools
            (pkgs.python3.withPackages (p:
              with p; [
                cffi
              ]))
          ];
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            mingw_w64
            mingw_w64_pthreads_w_static
          ]);
        };
        devShells.macos = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            pkgsCross.aarch64-darwin.buildPackages.gccStdenv
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "cargo" "rustc"];
            })
            gcc
          ];

          # RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
          #   extensions = ["rust-src"];
          # }}/lib/rustlib/src/rust/library";
          # PYO3_MINGW_DLLTOOL="${pkgs.llvmPackages.bintools-unwrapped}/bin/llvm-dlltool";
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
        python3 = prev.python3.override (pythonOverride prev);
        python310 = prev.python310.override (pythonOverride prev);
        python39 = prev.python39.override (pythonOverride prev);
      };
    };
}
