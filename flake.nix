{
  description = "Regionoix deployment & development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    deploy-rs.url = "github:serokell/deploy-rs";
  };

  outputs = {
    self,
    nixpkgs,
    deploy-rs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    nixosConfigurations = {
      Ravioles1 = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [];
      };
    };

    deploy.nodes = {
      Ravioles1 = {
        hostname = "regionoix.gasdev.fr";
        profiles.system = {
          user = "root";
          sshUser = "root";
          sshOpts = ["-p" "22"];
          sudo = "";
          path = deploy-rs.lib.x86_64-linux.activate.nixos self.nixosConfigurations.Raviole1;
        };
      };
    };

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
    #
    # This is highly advised, and will prevent many possible mistakes
    checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
  };
}
