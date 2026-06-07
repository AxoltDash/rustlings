{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    gcc
  ];

  shellHook = ''
    export NIX_ENFORCE_PURITY=0
    rustlings
  '';
}
