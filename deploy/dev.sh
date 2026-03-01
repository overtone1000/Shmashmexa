#!/bin/bash

CONTAINER_NAME=sh_dev

podman stop $CONTAINER_NAME
podman run --publish 30125:30125 --publish 4430:4430 --name $CONTAINER_NAME --replace --env DEVELOPMENT_MODE=false shmashmexa