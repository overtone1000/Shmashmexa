# Faux Show
A web application to serve as a simple home smart display device.

## Home Assistant Modifications

### Bypass Login

### Kiosk Mode
[Kiosk mode](https://github.com/NemesisRE/kiosk-mode)
If top bar is hidden, need to access dash with `?disable_km` at the end of the URL to enable editing.
http://10.10.10.10:8123/dashboard-kiosk/0?disable_km

### Restart device without reboot
```
sudo systemctl restart faux-show-backend cage-tty1
```

### Update tabs
ssh into device and 
`sudo nano /var/lib/containers/storage/volumes/faux_show_config/_data/tabs.json`

### Trying to sleep screen
```
XDG_RUNTIME_DIR=/run/user/$(id -u) #Should be kiosk user doing this? Yes!!
WAYLAND_DISPLAY=wayland-0 #Which display? Seems this works.
wlr-randr --help #Should be sudo? No! Should be run as kiosk!
wlr-randr --output HDMI-A-1 --off #Turns it off!
wlr-randr --output HDMI-A-1 --on #Turns it on!
```
Implemented in the `device` section of the library

### Photoprism API

Docs are here: https://docs.photoprism.dev/

Generate a token in the running container via cli: `photoprism auth add --name faux_show --scope read`

```
USER=admin
PHOTOPRISM_KEY=#Fill with webdav scoped photoprism key
URL=https://photos.overdesigned.org/api/v1

curl -H "Authorization: Bearer $PHOTOPRISM_KEY" -H "Content-Type: application/json" $URL/photos?count=2 #This works with  permissive app password

curl -X GET -H "X-Auth-Token: $PHOTOPRISM_KEY" $URL/photos?count=5 -H "accept: application.json"
```


### Testing Commands

```
URL=https://127.0.0.1:8443 #For local debug mode
URL=https://10.10.10.16:443 #For device
USER=faux_show_test_user#Fill with external user name
PASSWORD=faux_show_test_password#Fill with external user password

#Get messages with serialization test and add escapes
MESSAGE={\"ChangeDash\":{\"index\":3}} #Test changedash
MESSAGE={\"SetScreenState\":true} #Test screenstate

curl --insecure --user "$USER:$PASSWORD" -X POST -H "Content-Type: application/json" -d "message=$MESSAGE" $URL
```