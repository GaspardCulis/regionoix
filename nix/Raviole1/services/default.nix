{...}: {
  imports = [
    ./backend.nix
    ./caddy.nix # Proxy
    ./postgres.nix # DB
    ./redis.nix # Session storage
    ./web.nix # Web server
  ];
}
