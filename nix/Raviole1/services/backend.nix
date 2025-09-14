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
  sops.secrets."backend/meilisearch/admin_key".owner = "regionoix";
  sops.secrets."backend/meilisearch/search_key".owner = "regionoix";
  sops.secrets."backend/s3/access_key".owner = "regionoix";
  sops.secrets."backend/s3/secret_access_key".owner = "regionoix";
  sops.templates."regionoix-backend.env" = {
    content = ''
      DATABASE_URL=${config.sops.placeholder."backend/database_url"}
      SECRET_KEY=${config.sops.placeholder."backend/secret_key"}

      MEILISEARCH_ADMIN_KEY=${config.sops.placeholder."backend/meilisearch/admin_key"}
      MEILISEARCH_SEARCH_KEY=${config.sops.placeholder."backend/meilisearch/search_key"}

      S3_ACCESS_KEY=${config.sops.placeholder."backend/s3/access_key"}
      S3_SECRET_ACCESS_KEY=${config.sops.placeholder."backend/s3/secret_access_key"}
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

      S3_ENDPOINT_URL = "http://${config.services.garage.settings.s3_api.api_bind_addr}";
      S3_WEB_ENDPOINT_URL = "https://s3web.regionoix.gasdev.fr";
      S3_REGION = "${config.services.garage.settings.s3_api.s3_region}";
      S3_BUCKET_NAME = "images-bucket";
    };
  };
}
