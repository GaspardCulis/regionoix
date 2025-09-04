{
  description = "Regionoix deployment & development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    devShells.${system}.default = let
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
      };
  };
}
