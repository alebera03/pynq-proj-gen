#!/bin/bash

# $1 is abs path of .env to scan

# save envs
set -a
source "$1"
set +a
echo "envs saved"

# check if dir already exists
ssh -p "$REMOTE_PORT" "xilinx@$REMOTE_IP" "mkdir -p $REMOTE_PROJECT_PATH"
echo "remote folder: $REMOTE_PROJECT_PATH is ready"

# sync files
git add "$LOCAL_PROJECT_PATH"
rsync -avz -h -P \
    --exclude='.git' \
    --filter=':- .gitignore' -e "ssh -p $REMOTE_PORT" \
    "$LOCAL_PROJECT_PATH/" "xilinx@$REMOTE_IP:$REMOTE_PROJECT_PATH"