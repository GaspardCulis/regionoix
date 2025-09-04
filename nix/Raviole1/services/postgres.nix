{pkgs, ...}: let
  user = "regionoix";
  database = "regionoix";
  port = 5432;
  pass = "SCRAM-SHA-256$4096:R6A43wF+9je6l8/DkrCjLw==$XIwdnPNhz53tKWRHmBAAAgPOrOeTFSMEsVs+qH4SLsA=:joj8UbTHadOPpVHq0xSIp45LnPnqN7oMCkxOUw+8tN4=";
in {
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
    initialScript = pkgs.writeText "init-sql-script" ''
      alter user ${user} with password '${pass}';
      grant ALL on database ${database} to ${user};
    '';
  };

  networking.firewall.allowedTCPPorts = [port];
}
