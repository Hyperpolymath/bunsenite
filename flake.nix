{
  description = "Bunsenite: Nickel configuration file parser with multi-language FFI bindings";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Build inputs common to all derivations
        commonBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
        ];

        # Development tools
        devTools = with pkgs; [
          just
          cargo-edit
          cargo-audit
          cargo-outdated
          cargo-tarpaulin
          cargo-deny
          wasm-pack
          jq
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = commonBuildInputs ++ devTools;

          shellHook = ''
            echo "🔧 Bunsenite Development Environment"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  just --list      List all build recipes"
            echo "  just build       Build release binaries"
            echo "  just test        Run test suite"
            echo "  just check       Run all quality checks"
            echo "  just rsr-check   Verify RSR compliance"
            echo ""
            echo "RSR Compliance: Bronze Tier"
            echo "TPCF Perimeter: 3 (Community Sandbox)"
            echo "License: Dual MIT + Palimpsest 0.8"
            echo ""
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };

        # Package derivation
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "bunsenite";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = commonBuildInputs;

          buildInputs = with pkgs; [
            openssl
          ];

          # Run tests during build
          checkPhase = ''
            cargo test --release
          '';

          doCheck = true;

          meta = with pkgs.lib; {
            description = "Nickel configuration file parser with multi-language FFI bindings";
            homepage = "https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite";
            license = with licenses; [ mit ]; # Dual MIT + Palimpsest
            maintainers = [ "Campaign for Cooler Coding and Programming" ];
            platforms = platforms.all;
          };
        };

        # WASM package
        packages.wasm = pkgs.stdenv.mkDerivation {
          pname = "bunsenite-wasm";
          version = "0.1.0";

          src = ./.;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            wasm-pack
          ];

          buildPhase = ''
            wasm-pack build --target web --out-dir pkg --release
          '';

          installPhase = ''
            mkdir -p $out
            cp -r pkg/* $out/
          '';

          meta = with pkgs.lib; {
            description = "Bunsenite WebAssembly module";
            homepage = "https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite";
            license = with licenses; [ mit ];
          };
        };

        # CI check (runs all quality checks)
        packages.ci-check = pkgs.writeShellScriptBin "ci-check" ''
          set -e
          echo "Running CI checks..."

          echo "1. Format check..."
          cargo fmt --all -- --check

          echo "2. Clippy..."
          cargo clippy --all-targets --all-features -- -D warnings

          echo "3. Test suite..."
          cargo test --all-features

          echo "4. Unsafe code check..."
          if grep -r "unsafe" src/; then
            echo "ERROR: Found unsafe code!"
            exit 1
          fi

          echo "5. RSR compliance..."
          just rsr-check

          echo "✓ All CI checks passed!"
        '';

        # Apps
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        # Formatter
        formatter = pkgs.nixpkgs-fmt;

        # Checks (run with `nix flake check`)
        checks = {
          # Build check
          build = self.packages.${system}.default;

          # Format check
          format = pkgs.runCommand "check-format" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo fmt --all -- --check
            touch $out
          '';

          # Clippy check
          clippy = pkgs.runCommand "check-clippy" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo clippy --all-targets --all-features -- -D warnings
            touch $out
          '';

          # Test check
          test = pkgs.runCommand "check-tests" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo test --all-features
            touch $out
          '';
        };
      }
    );
}
