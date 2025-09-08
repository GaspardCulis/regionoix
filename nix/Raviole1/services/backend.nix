{
  pkgs,
  config,
  domain,
  ...
}: let
  regionoix-backend = pkgs.rustPlatform.buildRustPackage {
    pname = "regionoix-backend";
    version = "0.1.0";
    src = ../../..;

    cargoLock = {
      lockFile = ../../../Cargo.lock;
    };

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      openssl
    ];
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
    handle /api/*  {
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
