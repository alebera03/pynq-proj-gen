#!/bin/bash

file="./main.py"
if [[ -n $1 ]]; then
    file=$1
fi

# save envs
set -a
source .env
set +a

# check if dir already exists
ssh xilinx@169.254.91.19 "mkdir -p $REMOTE_PROJECT_PATH"

# sync files
rsync -avz --delete --exclude .env \
            --exclude run-remote.sh \
            --exclude run.sh \
            ./ xilinx@169.254.91.19:$REMOTE_PROJECT_PATH

# run pseudo-terminal shell attaching run-remote.sh
ssh -t xilinx@169.254.91.19 "cd $REMOTE_PROJECT_PATH && bash -c '$(cat run-remote.sh)' -- $file"