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

echo "envs saved"

# check if dir already exists
ssh xilinx@$REMOTE_IP "mkdir -p $REMOTE_PROJECT_PATH"

echo "remote folder is ready"

# sync files
rsync -avz --delete --exclude .scripts \
                    --exclude loaded.xclbin \
            .. xilinx@$REMOTE_IP:$REMOTE_PROJECT_PATH
# NOTE: '..' means that now rsync's POV IS '$CURRENT_DIR' and not '$CURRENT_DIR/.scripts',
#           so we want to exclude '.scripts' (NOT '.')


echo "running ssh remote terminal"

# run pseudo-terminal shell attaching run-remote.sh
ssh -t xilinx@$REMOTE_IP "cd $REMOTE_PROJECT_PATH && bash -c '$(cat run-remote.sh)' -- $file"

cd $CURRENT_DIR