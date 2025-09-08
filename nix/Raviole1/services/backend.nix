{
  pkgs,
  config,
  domain,
  ...
}: let
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

  regionoix-backend = pkgs.rustPlatform.buildRustPackage {
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
  };

  port = 6969;
in {
  sops.secrets."backend/database_url".owner = "regionoix";
  sops.secrets."backend/secret_key".owner = "regionoix";
  sops.templates."regionoix-backend.env" = {
    content = ''
      DATABASE_URL=${config.sops.placeholder."backend/database_url"}
      SECRET_KEY=${config.sops.placeholder."backend/secret_key"}
    '';
    owner = "regionoix";
  };

  services.caddy.virtualHosts."www.${domain}".extraConfig = ''
    handle /api/* {
      reverse_proxy 127.0.0.1:${toString port}
    }

    handle /api-docs/* {
      reverse_proxy 127.0.0.1:${toString port}
    }
  '';

  users.users.regionoix = {
    name = "regionoix";
    group = "regionoix";
    description = "Regionoix server user";
    isSystemUser = true;
  };
  users.groups.regionoix.name = "regionoix";

  systemd.services.regionoix-backend = {
    description = "The regionoix Rust backend";
    wants = ["network-online.target"];
    after = ["network-online.target"];
    wantedBy = ["multi-user.target"];
    enable = true;
    serviceConfig = {
      User = "regionoix";
      Group = "regionoix";
      Restart = "always";
      ExecStart = "${regionoix-backend}/bin/regionoix-backend";
      EnvironmentFile = config.sops.templates."regionoix-backend.env".path;
    };
    environment = {
      API_HOST = "127.0.0.1";
      API_PORT = toString port;
      REDIS_URL = "redis://127.0.0.1:6379";
    };
  };
}
