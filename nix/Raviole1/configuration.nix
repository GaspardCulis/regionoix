{...}: {
  imports = [
    ./hardware-configuration.nix
  ];

  networking.hostName = "Raviole1";
  # Firewall
  networking.nftables.enable = true;
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [22];
  };

  # SSH
  services = {
    openssh = {
      enable = true;
      ports = [22];
      settings = {
        PasswordAuthentication = false;
      };
    };
    fail2ban = {
      enable = true;
      maxretry = 5;
      bantime = "10m";
      bantime-increment.enable = true;
    };
  };

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHQyRXFQ6iA5p0vDuoGSHZfajiVZPAGIyqhTziM7QgBV gaspard@nixos"
  ];
}
