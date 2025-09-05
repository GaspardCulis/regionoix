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

      npmDepsHash = "sha256-VGUJYhy7F2nlB7XdKDcGFPFYx1hb1IaIqNrTUoeEe0s=";
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
