#!/bin/bash

# check if dir already exists
ssh -p $REMOTE_PORT xilinx@$REMOTE_IP "mkdir -p $REMOTE_PROJECT_PATH"
echo "remote folder: $REMOTE_PROJECT_PATH is ready"

# sync files
git add ..
rsync -avz -h -P \
    --delete \
    --exclude='.git/' \
    --filter=':- .gitignore' -e "ssh -p $REMOTE_PORT" \
    .. xilinx@$REMOTE_IP:$REMOTE_PROJECT_PATH
# NOTE: '..' means that now rsync's POV IS '$CURRENT_DIR' and not '$CURRENT_DIR/.scripts',
#           so we want to exclude '.scripts' (NOT '.')

ssh -p $REMOTE_PORT xilinx@$REMOTE_IP "cd $REMOTE_PROJECT_PATH"