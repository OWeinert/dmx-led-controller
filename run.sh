#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
# set -o xtrace # for debugging

readonly TARGET_HOST=pi@rpi.local
readonly TARGET_DIR=/home/pi/
readonly SOURCE_BIN=./target/aarch64-unknown-linux-gnu/debug/led-matrix-controller

cross_compile_and_sync() {
    cargo build --config=CrossCompileConfig.toml
    rsync ${SOURCE_BIN} ${TARGET_HOST}:${TARGET_DIR}
}

case "${1-""}" in
    # cross compile and deploy to rpi
    -d|--deploy)
        cross_compile_and_sync
        # exec bin
        ssh -t ${TARGET_HOST} sudo ${TARGET_DIR}/led-matrix-controller
        ;;
    # start gdbserver on rpi for remote debugging
    -g|--gdbserver)
        cross_compile_and_sync
        # start debug server
        ssh -t ${TARGET_HOST} gdbserver :1234 /home/pi/led-matrix-controller
        ;;
    # commands to run on rpi
    *)
        cargo build
        sudo ./target/debug/led-matrix-controller
        ;;
esac
