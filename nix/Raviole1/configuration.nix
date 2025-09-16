{
  lib,
  pkgs,
  ...
}: {
  imports = [
    ./hardware-configuration.nix
    ./services
  ];

  networking.hostName = "Raviole1";
  # Firewall
  networking.nftables.enable = true;
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [22];
  };

  environment.systemPackages = with pkgs;
    map lib.lowPrio [
      curl
      gitMinimal
      helix
      htop
      bottom
    ];

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
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCf4TCzQTBE0jPxeEs1UfpOjc4jbXOwskoPniWUyJj0UTjABGeBhR3mDL0wfUjT23vdD6JNIylPzs3X/27hiy1QTSMe/Kxdt2y6iRXp5ClgVXT4+aWDTd0zmSxEUb9GAra8n8P2+VGTC1EKd0r7O+hwmxSmIagpj5XB8rfcMFwifHrADOFFsJLYg572K2NgcjuEUu5bgv9TN8FemdlxBrL/5q2oQWwlyta7jBUVzJfLlz3e7RBp1Jt6fnoJSQOcJ5Emup+AgwDHHbzVBiG1cxB0BT/5TaPRokvQmDqW/rYS3YYSVOtzVohaazJRVx/rPSMMXTRE5u7vm9tsFQJ3LfJAa5VCer8kEG+hn7P+vZPeK1YPYoe15MZw4vVC/oNUa7XsyLW0tVreme8N/dHmcHU5iiqDVUP6C3ycajpusJmEx2vb/yEKcl8f/lkP623xv1SDmBFDJ+Iu1o2ja5iYSVkDFKO1VXT06s8IHUqKtinY5PJIQSfRyhbAKMD2ok+d/Bc= clavanna@im2ag"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ2hKyl4KJh7VmmhagDo4dgb9tfYc1dWJqQFEEqDD7mE zakaria@ZAKZOUK-PC"
  ];

  # SOPS
  sops.defaultSopsFile = ../secrets/Raviole1.yaml;
  sops.age.sshKeyPaths = ["/etc/ssh/ssh_host_ed25519_key"];

  system.stateVersion = "25.05";
}
