## Wifi (wpa supplicant wireless configuration)

Wifi network configuration is best achieved with the `nmtui` cli. It does persist through reboot, but Raspberry Pi (maybe other systems?) won't connnect if ethernet is connected.

Good idea to configure a 2.4 GHz AP as well.

## Updating

In this directory, run:
`sudo nixos-rebuild switch --ask-sudo-password --no-reexec --flake ./#rpi5 --sudo --target-host $USER$@$DEVICE_IP`

May need
`sudo nix --extra-experimental-features 'nix-command flakes' flake update` to force some things to update
    -Needed to update kiosk user UID
    -Needed to update systemd service? Or is __disabling import of service and then reenabling__ is what actually did it? Did both simultaneously, so not sure.