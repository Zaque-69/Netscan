{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    pkg-config
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}