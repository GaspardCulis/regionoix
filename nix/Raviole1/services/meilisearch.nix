{
  pkgs,
  config,
  ...
}: let
  port = 7700;
in {
  sops.secrets."meilisearch/master_key".owner = "meilisearch";

  users.users.meilisearch = {
    name = "meilisearch";
    group = "meilisearch";
    isSystemUser = true;
  };
  users.groups.meilisearch.name = "meilisearch";

  services.meilisearch = {
    enable = true;
    listenPort = port;
    masterKeyFile = config.sops.secrets."meilisearch/master_key".path;
  };

  # User fix
  systemd.services.meilisearch.serviceConfig = {
    User = "meilisearch";
    Group = "meilisearch";
    DynamicUser = pkgs.lib.mkForce false;
  };
}
