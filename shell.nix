{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.rustfmt
    pkgs.nodejs_20
    pkgs.pnpm
    pkgs.cargo
    pkgs.cargo-watch
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
