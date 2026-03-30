#!/bin/bash

# ./main.py because run-remote.sh use it directly in workdir ./ and not ./.scripts
file="./main.py"
if [[ -n $1 ]]; then
    file=$1
fi

CURRENT_DIR=$(pwd)
cd "$CURRENT_DIR/.scripts"

# save envs
set -a
source ./.env
set +a

# check if dir already exists
ssh xilinx@192.168.2.99 "mkdir -p $REMOTE_PROJECT_PATH"

# sync files
rsync -avz --delete --exclude ./.scripts \
            .. xilinx@192.168.2.99:$REMOTE_PROJECT_PATH

# run pseudo-terminal shell attaching run-remote.sh
ssh -t xilinx@192.168.2.99 "cd $REMOTE_PROJECT_PATH && bash -c '$(cat run-remote.sh)' -- $file"

cd $CURRENT_DIR