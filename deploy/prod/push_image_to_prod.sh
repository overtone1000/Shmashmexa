#!/bin/bash

set -e

source "./constants.sh"

PODMAN_HOST="ssh://$SSH_DEST:22/run/podman/podman.sock"

#Function to push an image
push_image_to_prod ( ) {
    
    echo "Removing any priors"
    rm --force /tmp/$IMAGE_NAME
    ssh -l $USER $SERVER_IP "rm --force /tmp/$IMAGE_NAME"

    echo "Creating archive"
    podman save --format oci-archive --output /tmp/$IMAGE_NAME $IMAGE_NAME:latest

    echo "Transferring archive"
    scp -v -r /tmp/$IMAGE_NAME $SSH_DEST:/tmp/$IMAGE_NAME

    echo "Pulling image from archive on remote"
    #podman --remote --url $PODMAN_HOST load --input /tmp/$IMAGE_NAME
    ssh -l $USER $SERVER_IP "podman load --input /tmp/$IMAGE_NAME"

    #echo "Cleaning up"
    #ssh -l $USER $SERVER_IP "rm /tmp/$IMAGE_NAME"
}

#Push image
push_image_to_prod