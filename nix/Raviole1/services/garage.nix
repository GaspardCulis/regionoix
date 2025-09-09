{
  config,
  domain,
  ...
}: let
  apiPort = 3900;
  rpcPort = 3901;
  webPort = 3902;

  apiDomain = "s3.${domain}";
  webDomain = "s3web.${domain}";
in {
  sops.secrets."garage/rpc_secret".owner = "root";

  services.caddy.virtualHosts."${apiDomain}".extraConfig = ''
    header {
      ?Access-Control-Allow-Headers *
      ?Access-Control-Allow-Methods *
      ?Access-Control-Allow-Origin *
    }
    reverse_proxy http://127.0.0.1:${toString apiPort}
  '';

  services.caddy.virtualHosts."${webDomain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString webPort}
  '';

  services.garage = {
    enable = true;
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
        root_domain = "s3.${domain}";
      };
      s3_web = {
        bind_addr = "[::]:${toString webPort}";
        root_domain = ".${domain}";
        index = "index.html";
      };
    };
  };
}
