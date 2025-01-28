{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { flake-utils
    , naersk
    , nixpkgs
    , fenix
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk { };

        baseToolchain = pkgs.fenix.stable.withComponents [
          "cargo"
          "clippy"
          "llvm-tools"
          "rustc"
          "rustfmt"
        ];

        targetToolchain = pkgs.fenix.combine [
          baseToolchain
          (pkgs.fenix.fromToolchainFile {
            dir = ./.;
            sha256 = "sha256-lMLAupxng4Fd9F1oDw8gx+qA0RuF7ou7xhNU8wgs0PU=";
          })
        ];
      in
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-analyzer
            targetToolchain
            dioxus-cli
            pkg-config
            openssl
          ];
        };
      }
    );
}
