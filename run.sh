#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
# set -o xtrace # for debugging

readonly TARGET_HOST=pi@rpi.local
readonly TARGET_PATH=/home/pi/led-matrix-controller
readonly SOURCE_PATH=./

rsync -a ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

ssh -tt ${TARGET_HOST} << EOF
sudo killall led-matrix-controller
cd ${TARGET_PATH}
cargo build
sudo ./target/debug/led-matrix-controller
exit
EOF
