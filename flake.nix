{
  description = "Axdot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    (flake-utils.lib.eachDefaultSystem

      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        rec {
          packages.axdot = pkgs.callPackage ./default.nix { };
          defaultPackage = packages.axdot;
          apps.axdot = flake-utils.lib.mkApp {
            drv = packages.axdot;
            exePath = "/bin/axdot";
          };
          defaultApp = apps.axdot;
          devShell = pkgs.callPackage ./shell.nix { };

        })) // {
      overlay = final: prev: {
        axdot = final.callPackage ./default.nix { };
      };
    };
}
