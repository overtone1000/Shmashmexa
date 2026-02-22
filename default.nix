# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [
        nodejs
        #pkg-config 
        #dbus 
        #openssl 
        #glib 
        #gtk3 
        #libsoup_3
        #webkitgtk_4_1
        #appimagekit 
    ];
}