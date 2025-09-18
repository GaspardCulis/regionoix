{pkgs ? import <nixpkgs> {}}:
pkgs.buildNpmPackage {
  pname = "regionoix-frontend";
  version = "0.1.0";

  src = ../../..;

  npmDepsHash = "sha256-M8gJPRVK6kWt41mGNxLERSt7zudJU3YstB0u2981SNU=";
  npmBuildScript = "build";

  installPhase = ''
    runHook preInstall
    cp -pr --reflink=auto dist $out/
    runHook postInstall
  '';
}
