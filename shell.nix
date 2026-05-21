{pkgs ? import <nixpkgs> {}}: let
  inherit (pkgs) mkShell helix rustc cargo rust-analyzer alejandra rustfmt;
in
  mkShell {
    packages = [
      helix
      rustc
      cargo
      rust-analyzer
      alejandra
      rustfmt
    ];
  }
