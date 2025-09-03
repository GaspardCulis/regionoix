let
  rust_overlay = import (builtins.fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "4178888556c15e0a1c57850d2f103ac300a6e9e2";
  });
  pkgs = import <nixpkgs> {overlays = [rust_overlay];};
  rustVersion = "latest";
  rustToolchain = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.channel;
  rust = pkgs.rust-bin.${rustToolchain}.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
      "llvm-tools-preview"
    ];
  };
in
  pkgs.mkShell rec {
    packages = with pkgs; [
      # Front
      nodejs
      # Back
      cargo-llvm-cov
    ];
    nativeBuildInputs = with pkgs; [
      rust
      pkg-config
    ];
    buildInputs = [
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
  }
