{
  pkgs,
  config,
  ...
}: let
  port = 7700;
in {
  sops.secrets."meilisearch/master_key".owner = "meilisearch";
  sops.templates."meilisearch-masterkey.env" = {
    content = ''
      MEILI_MASTER_KEY=${config.sops.placeholder."postgres/password"}
    '';
    owner = "meilisearch";
  };

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

  # User fix
  systemd.services.meilisearch.serviceConfig = {
    User = "meilisearch";
    Group = "meilisearch";
    DynamicUser = pkgs.lib.mkForce false;
  };
}
