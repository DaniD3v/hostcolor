{
  description = "Stable color derived from a string, the hostname by default";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "hostcolor";
          version = "0.1.0";

          src = self;
          cargoLock.lockFile = ./Cargo.lock;

          meta.mainProgram = "hostcolor";
        };
      }
    );
}
