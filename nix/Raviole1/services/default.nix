{...}: {
  imports = [
    ./backend.nix
    ./caddy.nix # Proxy
    ./garage.nix # S3 storage
    ./meilisearch.nix # Search indexer
    ./postgres.nix # DB
    ./redis.nix # Session storage
    ./web.nix # Web server
  ];
}
