{
  description = "A fedimint based lightning address server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        flakeboxLib = flakebox.lib.${system} { };
        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "hermes";
            path = ./.;
          };
          paths = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };

        toolchainArgs = {
          extraRustFlags = "--cfg tokio_unstable";
        } // lib.optionalAttrs pkgs.stdenv.isDarwin {
          # on Darwin newest stdenv doesn't seem to work
          # linking rocksdb
          stdenv = pkgs.clang11Stdenv;
        };

        # all standard toolchains provided by flakebox
        toolchainsStd =
          flakeboxLib.mkStdFenixToolchains toolchainArgs;

        toolchainsNative = (pkgs.lib.getAttrs
          [
            "default"
          ]
          toolchainsStd
        );

        toolchainNative = flakeboxLib.mkFenixMultiToolchain {
          toolchains = toolchainsNative;
        };

        commonArgs = {
          buildInputs = [
            pkgs.openssl
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
        };
        outputs = (flakeboxLib.craneMultiBuild { toolchains = toolchainsStd; }) (craneLib':
          let
            craneLib = (craneLib'.overrideArgs {
              pname = "flexbox-multibuild";
              src = rustSrc;
            }).overrideArgs commonArgs;
          in
          rec {
            workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
            workspaceBuild =
              craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
            hermes = craneLib.buildPackageGroup
              { pname = "hermes"; packages = [ "hermes" ]; mainProgram = "hermes"; };
          });
      in
      {
        legacyPackages = outputs;
        packages = {
          default = outputs.hermes;
        };
        devShells = flakeboxLib.mkShells (commonArgs // {
          toolchain = toolchainNative;
        });
      });
}
