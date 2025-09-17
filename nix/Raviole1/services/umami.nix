{
  config,
  domain,
  ...
}: let
  port = 4341;
in {
  sops.secrets."umami/app_secret".owner = "root";
  sops.secrets."umami/db_user".owner = "root";
  sops.secrets."umami/db_pass".owner = "root";
  sops.templates."umami.env" = {
    content = ''
      APP_SECRET=${config.sops.placeholder."umami/app_secret"}
      DATABASE_URL=postgresql://${config.sops.placeholder."umami/db_user"}:${config.sops.placeholder."umami/db_pass"}@umami-db:5432/umami
    '';
    owner = "docker";
  };
  sops.templates."umami-db.env" = {
    content = ''
      POSTGRES_USER=${config.sops.placeholder."umami/db_user"}
      POSTGRES_PASSWORD=${config.sops.placeholder."umami/db_pass"}
    '';
    owner = "docker";
  };

  services.caddy.virtualHosts."analytics.${domain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString port}
  '';

  virtualisation.oci-containers.containers = {
    umami = {
      image = "ghcr.io/umami-software/umami:postgresql-latest";
      autoStart = true;
      ports = ["127.0.0.1:${toString port}:3000"];
      dependsOn = ["umami-db"];
      environment = {
        DATABASE_TYPE = "postgresql";
      };
      environmentFiles = [
        config.sops.templates."umami.env".path
      ];
    };
    umami-db = {
      image = "docker.io/postgres:15-alpine";
      autoStart = true;
      environment = {
        POSTGRES_DB = "umami";
      };
      environmentFiles = [
        config.sops.templates."umami-db.env".path
      ];
      volumes = [
        "umami-db-data:/var/lib/postgresql/data"
      ];
    };
  };
}
