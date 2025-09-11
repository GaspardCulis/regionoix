{
  config,
  domain,
  pkgs,
  ...
}: let
  port = 7700;
in {
  sops.secrets."meilisearch/master_key".owner = "meilisearch";
  sops.templates."meilisearch-masterkey.env" = {
    content = ''
      MEILI_MASTER_KEY=${config.sops.placeholder."meilisearch/master_key"}
    '';
    owner = "meilisearch";
  };

  services.caddy.virtualHosts."search.${domain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString port}
  '';

  users.users.meilisearch = {
    name = "meilisearch";
    group = "meilisearch";
    isSystemUser = true;
  };
  users.groups.meilisearch.name = "meilisearch";

  services.meilisearch = {
    enable = true;
    listenPort = port;
    environment = "production";
    masterKeyEnvironmentFile = config.sops.templates."meilisearch-masterkey.env".path;
  };

  # Override default DynamicUser to use custom meilisearch user and group for proper access to SOPS-managed secrets
  systemd.services.meilisearch.serviceConfig = {
    User = "meilisearch";
    Group = "meilisearch";
    DynamicUser = pkgs.lib.mkForce false;
  };
}
