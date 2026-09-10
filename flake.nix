{
  description = "Klownie Portfolio";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        wasm-bindgen-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "wasm-bindgen-cli";
          version = "0.2.128";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-a7lcXJnnZkYReja+iUO7NqqrWyv3toxnUgQb8s4IS5s=";
          };

          cargoHash = "sha256-R1Tas33Ursy8kqsxguAkG0ZhNed2n5uFTAhw1l2qlLY=";

          meta = {
            description = "CLI tool for wasm-bindgen";
            homepage = "https://github.com/wasm-bindgen/wasm-bindgen";
          };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            cargo-leptos
            lld
            wasm-pack
            dart-sass
            binaryen
            nodejs
            wasm-bindgen-cli
          ];
        };
      }
    );
}
