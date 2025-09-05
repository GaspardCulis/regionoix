{...}: {
  services.redis.servers."session-storage" = {
    enable = true;
    bind = "127.0.0.1";
    port = 6379;
  };
}
