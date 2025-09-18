{...}: {
  imports = [
    ./backend.nix
    ./beszel.nix # Server monitoring
    ./caddy.nix # Proxy
    ./garage.nix # S3 storage
    ./meilisearch.nix # Search indexer
    ./postgres.nix # DB
    ./redis.nix # Session storage
    ./umami.nix # Website analytics
    ./uptime-kuma.nix # Service monitoring
    ./web.nix # Web server
  ];
}
