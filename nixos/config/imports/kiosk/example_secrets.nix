
{ config, pkgs, ... }:
let
    universal_keys = [ 
        #Put ssh keys for root and dev account here
    ];
in
{
  users.users.root = {
    openssh.authorizedKeys.keys  = universal_keys;
  };
  
  users.users.dev_account = {
    openssh.authorizedKeys.keys  = universal_keys;
  };

  nix.settings.trusted-users = [ "tydev_accountler" ];
	users.users = {
		dev_account = {
			isNormalUser = true;
			description = "Developer";
			extraGroups = [ 
				"networkmanager"
				"wheel" #root privileges
			];
		};
	};
}
