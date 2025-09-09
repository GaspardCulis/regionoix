{
  domain,
  pkgs,
  ...
}: let
  regionoix-frontend =
    pkgs.callPackage ../../pkgs/regionoix-frontend {}
    + "/regionoix/browser";
in {
  services.caddy.virtualHosts."www.${domain}".extraConfig = ''
    root * ${regionoix-frontend}
    file_server
  '';
}
