{domain, ...}: let
  port = 3001;
in {
  services.caddy.virtualHosts."uptime.${domain}".extraConfig = ''
    reverse_proxy http://127.0.0.1:${toString port}
  '';

  services.uptime-kuma = {
    enable = true;
  };
}
