#!/bin/bash

set -e

QUADLET_MEMBERS=(
    #Pod first
    shmashmexa-pod
    #Resources next
    shmashmexa_config-volume
    #Container builds
    shmashmexa-build
    #Containers in dependency order
    shmashmexa
    nginx
)

echo Quadlet members are: ${QUADLET_MEMBERS[@]}