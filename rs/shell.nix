{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ 
    rustc
    clippy
    rustfmt
    just
    cargo-nextest ];
}