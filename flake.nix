{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # taffy = {
    #   url = "path:/home/xavier/projects/taffy";
    # };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    let
      overlay = final: prev: {
        taffy-clib = final.callPackage ({pkgs}:
          let
            craneLib = crane.mkLib pkgs;
          in
          craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./src/taffy-clib;

          buildInputs = [
            # Add additional build inputs here
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          nativeBuildInputs = [
            pkgs.rust-cbindgen
          ];

          postInstall = ''
            cbindgen --lang c --output $out/include/taffy.h
          '';
        }) {};
      };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [overlay];
        };

        craneLib = crane.lib.${system};
        taffy-c = pkgs.stdenv.mkDerivation {
          name = "taffy-c";

          src = ./src/taffy-c;

          buildInputs = [ pkgs.taffy-clib ];

          configurePhase = ''
            declare -xp
          '';
          buildPhase = ''
            cc "$src/taffy-c.c" -o ./taffy-c -ltaffy -lm
          '';
          installPhase = ''
            mkdir -p "$out/bin"
            cp ./taffy-c "$out/bin/"
          '';
        };
      in
      {
        checks = {
          inherit (pkgs) taffy-clib;
        };

        packages.default = pkgs.taffy-clib;
        packages.taffy-clib = pkgs.taffy-clib;
        packages.taffy-c = taffy-c;

        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      }) // {
        inherit overlay;
      };
}
