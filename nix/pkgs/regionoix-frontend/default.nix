{pkgs ? import <nixpkgs> {}}:
pkgs.buildNpmPackage {
  pname = "regionoix-frontend";
  version = "0.1.0";

  src = ../../..;

  npmDepsHash = "sha256-JCtFAApj66tGS20zQf4Dlbb0JuXFHOl++Gt6ABr7hxQ=";
  npmBuildScript = "build";

  installPhase = ''
    runHook preInstall
    cp -pr --reflink=auto dist $out/
    runHook postInstall
  '';
}
