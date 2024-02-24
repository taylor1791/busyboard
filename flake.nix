{
  description = "Dataforge monorepo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  };

  outputs = { self, nixpkgs }: let
    inherit (nixpkgs) lib;
  in {
    devShell = lib.genAttrs lib.systems.flakeExposed (system: let
      pkgs = import nixpkgs { inherit system; };
    in pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo clippy rust-analyzer rustc rustfmt
      ];
    });
  };
}
