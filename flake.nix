{
  description = "A pretty ls replacement, inspired by the nushell ls.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
	cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "pretty-ls";
          version = cargoToml.package.version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/pls";
        };
      }
    );
}
