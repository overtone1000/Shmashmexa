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

Tried `photoprism auth` but permission constantly denied.

```
PHOTOPRISM_KEY=#Fill with photoprism key
SLIDESHOW_ALBUM_UID=#Fill with UID from photoprism for desired album
URL=https://photos.overdesigned.org/api/v1
EXAMPLE_PHOTO_UID=#Get an example photo and put uid here for testing
EXAMPLE_FILE_UID=#Get an exampl file uid

curl -X "GET" -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/albums?count=5 #Works, can browse albums

curl -X "GET" -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/albums/$SLIDESHOW_ALBUM_UID #Get the slideshow album details

curl -X "GET" -H "Authorization: Bearer $PHOTOPRISM_KEY" "$URL/photos?count=10&merged=true&public=true&s=$SLIDESHOW_ALBUM_UID" #Works! Need quotes for this queary

curl -X "GET" -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/photos/$EXAMPLE_PHOTO_UID #Works

curl -X "GET" --output test_image -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/photos/$EXAMPLE_PHOTO_UID/dl -H "accept: application/octet-stream" #Works, but just a short html with svg even if using a known image UID from web interface.

curl -X "GET" --output test_image -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/dl/$EXAMPLE_FILE_UID -H "accept: application/octet-stream" #Works, but just a short html with svg even if using a known image UID from web interface.

curl -X "GET" --output test_zip -H "Authorization: Bearer $PHOTOPRISM_KEY" $URL/albums/$SLIDESHOW_ALBUM_UID/dl -H "accept: application/zip"

#This gets a photo out! Where DOWNLOAD_TOKEN is the value in "downloadToken" from querying /session
curl --output test_file -X "GET" $URL/photos/$EXAMPLE_PHOTO_UID/dl?t=$DOWNLOAD_TOKEN  -H "accept: application/octet-stream"
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