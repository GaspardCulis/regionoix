{
  config,
  domain,
  pkgs,
  ...
}: let
  apiPort = 3900;
  rpcPort = 3901;
  webPort = 3902;

  apiDomain = "s3.${domain}";
  webDomain = "s3web.${domain}";
in {
  sops.secrets."garage/rpc_secret".owner = "garage";

  services.caddy.virtualHosts."${apiDomain}".extraConfig = ''
    header {
      ?Access-Control-Allow-Headers *
      ?Access-Control-Allow-Methods *
      ?Access-Control-Allow-Origin *
    }
    reverse_proxy http://127.0.0.1:${toString apiPort}
  '';

  services.caddy.virtualHosts."images-bucket.${webDomain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString webPort}
  '';

  users.users.garage = {
    name = "garage";
    group = "garage";
    isSystemUser = true;
  };
  users.groups.garage.name = "garage";

  services.garage = {
    enable = true;
    package = pkgs.garage_2;
    settings = {
      metadata_dir = "/var/lib/garage/meta";
      data_dir = "/var/lib/garage/data";
      db_engine = "lmdb";
      metadata_auto_snapshot_interval = "6h";

      replication_factor = 1;
      compression_level = 2;

      rpc_bind_addr = "[::]:${toString rpcPort}";
      rpc_public_addr = "127.0.0.1:${toString rpcPort}";
      rpc_secret_file = config.sops.secrets."garage/rpc_secret".path;

      s3_api = {
        s3_region = "garage";
        api_bind_addr = "[::]:${toString apiPort}";
        root_domain = "${apiDomain}";
      };
      s3_web = {
        bind_addr = "[::]:${toString webPort}";
        root_domain = "${webDomain}";
        index = "index.html";
      };
    };
  };

  # User fix
  systemd.services.garage.serviceConfig = {
    User = "garage";
    Group = "garage";
    DynamicUser = pkgs.lib.mkForce false;
  };
}
