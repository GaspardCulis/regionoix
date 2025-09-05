{...}: {
  imports = [
    ./postgres.nix # DB
    ./caddy.nix # Proxy
    ./web.nix # Web server
  ];
}
