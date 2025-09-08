{config, ...}: let
  user = "regionoix";
  database = "regionoix";
  port = 5432;
in {
  sops.secrets."postgres/password".owner = "postgres";
  sops.templates."postgres-init-sql-script" = {
    content = ''
      alter user ${user} with password '${config.sops.placeholder."postgres/password"}';
      grant ALL on database ${database} to ${user};
    '';
    owner = "postgres";
  };

  services.postgresql = {
    enable = true;
    enableTCPIP = true;
    ensureDatabases = [
      database
    ];
    ensureUsers = [
      {name = user;}
    ];
    settings = {
      inherit port;
    };
    authentication = ''
      host all ${user} 0.0.0.0/0 scram-sha-256
    '';
    initialScript = config.sops.templates."postgres-init-sql-script".path;
  };

  networking.firewall.allowedTCPPorts = [port];
}
