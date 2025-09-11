{
  pkgs,
  config,
  domain,
  ...
}: let
  regionoix-backend = pkgs.callPackage ../../pkgs/regionoix-backend {};
  port = 6969;
in {
  sops.secrets."backend/database_url".owner = "regionoix";
  sops.secrets."backend/secret_key".owner = "regionoix";
  sops.secrets."backend/meilisearch_admin_key".owner = "regionoix";
  sops.secrets."backend/meilisearch_search_key".owner = "regionoix";
  sops.templates."regionoix-backend.env" = {
    content = ''
      DATABASE_URL=${config.sops.placeholder."backend/database_url"}
      SECRET_KEY=${config.sops.placeholder."backend/secret_key"}

      MEILISEARCH_ADMIN_KEY=${config.sops.placeholder."backend/meilisearch_admin_key"}
      MEILISEARCH_SEARCH_KEY=${config.sops.placeholder."backend/meilisearch_search_key"}
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
      MEILISEARCH_URL = "http://127.0.0.1:${toString config.services.meilisearch.listenPort}";
    };
  };
}
