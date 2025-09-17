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

    @notFound {
      not path /api/*
      not path /api-docs/*
      not file {path}
    }

    rewrite @notFound /index.html
  '';
}
