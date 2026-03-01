#!/bin/bash

set -e

source "../commons/1 move_to_main_repo_dir.sh"

source "./deploy/commons/2 build_frontend.sh"

echo "This requires the trm-rust-libs repo to be in the parent directory of this repo."

#Change to parent directory of this repo to allow access to trm-rust-libs repo
cd ../..
#Build container, which builds backend inside container
podman build --file ./Shmashmexa/deploy/containerize/Containerfile --tag shmashmexa .