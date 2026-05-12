{
  description = "Client-side Fedimint decoder web app";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    # Reuse Fedimint's pinned nixpkgs, overlays, and binary cache setup.
    # Keep this aligned with the fedimint-core crate version in Cargo.toml.
    fedimint.url = "github:fedimint/fedimint/v0.11.1";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "fedimint/nixpkgs";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      fedimint,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import fedimint.inputs.nixpkgs {
          inherit system;
          overlays = [
            fedimint.overlays.darwin-compile-fixes
          ];
        };

        lib = pkgs.lib;
        llvm = pkgs.llvmPackages_20 or pkgs.llvmPackages;
        clangResourceIncludes = "${llvm.clang-unwrapped.lib}/lib/clang/${lib.versions.major llvm.clang-unwrapped.version}/include";
        fenixPkgs = fenix.packages.${system};
        wasmBindgenVersion = "0.2.121";
        wasmBindgenCli = pkgs.rustPlatform.buildRustPackage {
          pname = "wasm-bindgen-cli";
          version = wasmBindgenVersion;

          src = pkgs.fetchCrate {
            pname = "wasm-bindgen-cli";
            version = wasmBindgenVersion;
            hash = "sha256-ZOMgFNOcGkO66Jz/Z83eoIu+DIzo3Z/vq6Z5g6BDY/w=";
          };

          cargoHash = "sha256-DPdCDPTAPBrbqLUqnCwQu1dePs9lGg85JCJOCIr9qjU=";
          doCheck = false;
        };
        rustToolchain = fenixPkgs.combine [
          fenixPkgs.stable.toolchain
          fenixPkgs.targets.wasm32-unknown-unknown.stable.rust-std
        ];

        wasmEnv = {
          LIBCLANG_PATH = "${llvm.libclang.lib}/lib";
          # Use the unwrapped compiler for C code built by ring/secp256k1-sys.
          # The Nix cc-wrapper injects host hardening flags that are invalid for
          # wasm32-unknown-unknown.
          CC_wasm32_unknown_unknown = "${llvm.clang-unwrapped}/bin/clang";
          CXX_wasm32_unknown_unknown = "${llvm.clang-unwrapped}/bin/clang++";
          AR_wasm32_unknown_unknown = "${llvm.bintools-unwrapped}/bin/llvm-ar";
          RANLIB_wasm32_unknown_unknown = "${llvm.bintools-unwrapped}/bin/llvm-ranlib";
          CFLAGS_wasm32_unknown_unknown = "-I${clangResourceIncludes}";
          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "rust-lld";
        };

        nativeBuildInputs = [
          rustToolchain
          pkgs.trunk
          pkgs.just
          wasmBindgenCli
          pkgs.binaryen
          llvm.clang-unwrapped
          llvm.libclang
          llvm.bintools-unwrapped
        ];
      in
      {
        devShells.default = pkgs.mkShell (
          wasmEnv
          // {
            nativeBuildInputs = nativeBuildInputs ++ [
              pkgs.cargo-watch
              pkgs.git
              pkgs.just
              pkgs.nixfmt-rfc-style
            ];

            shellHook = ''
              export PATH="${llvm.bintools}/bin:$PATH"
              echo "Fedimint decoder WASM shell"
              echo "Run: trunk serve"
            '';
          }
        );

        packages.default = pkgs.rustPlatform.buildRustPackage (
          wasmEnv
          // {
            pname = "fedimint-decoder";
            version = "0.1.0";
            src = lib.cleanSource self;

            cargoLock.lockFile = ./Cargo.lock;

            inherit nativeBuildInputs;

            buildPhase = ''
              runHook preBuild
              export HOME="$TMPDIR"
              export PATH="${llvm.bintools}/bin:$PATH"
              trunk build --release --public-url .
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              cp -R dist/. "$out/"
              runHook postInstall
            '';

            doCheck = false;
          }
        );

        apps.default = {
          type = "app";
          program = "${pkgs.writeShellScript "serve-fedimint-decoder" ''
            set -euo pipefail
            exec ${pkgs.miniserve}/bin/miniserve ${self.packages.${system}.default} --index index.html
          ''}";
        };
      }
    );

  nixConfig = {
    allow-dirty = true;
    warn-dirty = false;
    extra-substituters = [
      "https://fedimint.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "fedimint.cachix.org-1:FpJJjy1iPVlvyv4OMiN5y9+/arFLPcnZhZVVCHCDYTs="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
