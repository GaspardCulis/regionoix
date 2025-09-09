{pkgs ? import <nixpkgs> {}}:
pkgs.buildNpmPackage {
  pname = "regionoix-frontend";
  version = "0.1.0";

  src = ../..;

  npmDepsHash = "sha256-v/M93gyHmFhUHa3oXHOJa3eRpI4TCSFfeENWGkix8L8=";
  npmBuildScript = "build";

  installPhase = ''
    runHook preInstall
    cp -pr --reflink=auto dist $out/
    runHook postInstall
  '';
}
