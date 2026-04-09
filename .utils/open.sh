#!/bin/bash

# $1 is abs path of .env to scan

# save envs
set -a
source $1
set +a
echo "envs saved"

exec ssh -t -p $REMOTE_PORT xilinx@$REMOTE_IP "cd $REMOTE_PROJECT_PATH && bash --login"