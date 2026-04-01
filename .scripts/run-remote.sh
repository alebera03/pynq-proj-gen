#!/bin/bash

# sourcing
[ -f /etc/profile ] && source /etc/profile
[ -f ~/.bashrc ] && source ~/.bashrc
[ -f /etc/profile.d/xrt_setup.sh ] && source /etc/profile.d/xrt_setup.sh

echo "sourcing succeeded"

echo "xilinx" | sudo -S chown -R xilinx:xilinx .
echo "xilinx" | sudo -S -E /usr/local/share/pynq-venv/bin/python $1