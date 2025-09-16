{pkgs ? import <nixpkgs> {}}:
pkgs.buildNpmPackage {
  pname = "regionoix-frontend";
  version = "0.1.0";

  src = ../../..;

  npmDepsHash = "sha256-x3aqnQZiOEXv6KKC7LC+IVH/cSJEL9zvF9VZp/HKwXk=";
  npmBuildScript = "build";

  installPhase = ''
    runHook preInstall
    cp -pr --reflink=auto dist $out/
    runHook postInstall
  '';
}
