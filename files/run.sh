#!/bin/bash

file="./main.py"
if [[ -n $1 ]]; then
    file=$1
fi

set -a
source .env
set +a

rsync -avz --delete --exclude .env \
            --exclude run-remote.sh \
            --exclude run.sh \
            ./ xilinx@169.254.91.19:$REMOTE_PROJECT_PATH
ssh -t xilinx@169.254.91.19 "cd $REMOTE_PROJECT_PATH && bash -c '$(cat run-remote.sh)' -- $file"