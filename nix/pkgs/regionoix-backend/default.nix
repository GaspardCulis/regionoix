{pkgs ? import <nixpkgs> {}}: let
  swagger-ui = pkgs.stdenv.mkDerivation rec {
    name = "swagger-ui";
    src = builtins.fetchurl {
      url = "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v${version}.zip";
      sha256 = "sha256:1p6cf4zf3jrswqa9b7wwgxhp3ca2v5qrzxzfp8gv35r0h78484j8";
    };

    version = "5.17.14";

    phases = ["installPhase"]; # Removes all phases except installPhase

    installPhase = ''
      mkdir -p $out/lib
      cp ${src} $out/lib/swagger-ui.zip
    '';
  };
in
  pkgs.rustPlatform.buildRustPackage {
    pname = "regionoix-backend";
    version = "0.1.0";
    src = ../../..;

    cargoLock = {
      lockFile = ../../../Cargo.lock;
    };

    doCheck = false;

    nativeBuildInputs = with pkgs; [
      pkg-config
      curl
    ];

    buildInputs = with pkgs; [
      openssl
      swagger-ui
    ];

    SWAGGER_UI_DOWNLOAD_URL = "file://${swagger-ui}/lib/swagger-ui.zip";
  }
