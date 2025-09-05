{
  domain,
  pkgs,
  ...
}: let
  regionoix-frontend =
    pkgs.buildNpmPackage {
      pname = "regionoix-frontend";
      version = "0.1.0";

      src = ../../..;

      npmDepsHash = "sha256-v/M93gyHmFhUHa3oXHOJa3eRpI4TCSFfeENWGkix8L8=";
      npmBuildScript = "build";

      installPhase = ''
        runHook preInstall
        cp -pr --reflink=auto dist $out/
        runHook postInstall
      '';
    }
    + "/regionoix/browser";
in {
  services.caddy.virtualHosts."www.${domain}".extraConfig = ''
    root * ${regionoix-frontend}
    file_server
  '';
}
