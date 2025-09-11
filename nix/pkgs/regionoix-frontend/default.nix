{pkgs ? import <nixpkgs> {}}:
pkgs.buildNpmPackage {
  pname = "regionoix-frontend";
  version = "0.1.0";

  src = ../../..;

  npmDepsHash = "sha256-lHjB7uD181tRm3aYQBYULxyYrnFxFqVD4pwPxg1U9NA=";
  npmBuildScript = "build";

  installPhase = ''
    runHook preInstall
    cp -pr --reflink=auto dist $out/
    runHook postInstall
  '';
}
