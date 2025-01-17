let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  libPkgs = with pkgs; [
      toolchain
      cargo-bootimage
      clippy
      rust-analyzer
      qemu
    ];
    libPath = pkgs.lib.makeLibraryPath libPkgs;
in
pkgs.mkShell {
  packages = libPkgs;
  LD_LIBRARY_PATH = libPath;
}
