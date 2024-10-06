{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = [ pkgs.rustc pkgs.rustfmt pkgs.cargo pkgs.nodejs_18 pkgs.pnpm ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
