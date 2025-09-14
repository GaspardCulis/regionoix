{
  domain,
  pkgs,
  ...
}: let
  agent = {
    port = 45876;
    key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIA10d/QOhoAkD0Ntoomsf/qj5BpZsk6EBRolCFMILOA3";
  };

  hub = {
    port = 8090;
  };
in {
  services.caddy.virtualHosts."monitor.${domain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString hub.port}
  '';

  systemd.services.beszel-agent = {
    enable = true;
    description = "Lightweight server monitoring agent";
    wants = ["network-online.target"];
    after = ["network-online.target"];
    wantedBy = ["multi-user.target"];
    serviceConfig = {
      Restart = "always";
      ExecStart = ''
        ${pkgs.beszel}/bin/beszel-agent -key "${agent.key}" -listen "127.0.0.1:${toString agent.port}"
      '';
    };
  };

  systemd.services.beszel-hub = {
    enable = true;
    description = "Lightweight server monitoring platform";
    wants = ["network-online.target"];
    after = ["network-online.target"];
    wantedBy = ["multi-user.target"];
    serviceConfig = {
      Restart = "always";
      DynamicUser = true;
      StateDirectory = "beszel";
      ExecStart = ''
        ${pkgs.beszel}/bin/beszel-hub serve --dir /var/lib/beszel --http 127.0.0.1:${toString hub.port}
      '';
    };
  };
}
