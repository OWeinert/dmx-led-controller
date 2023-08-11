#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
# set -o xtrace # for debugging

readonly TARGET_HOST=pi@rpi.local
readonly TARGET_DIR=/home/pi
readonly SOURCE_BIN=./target/aarch64-unknown-linux-gnu/debug/main

cross_compile_and_sync() {
    cargo build --bin main --package led-matrix-controller --config=CrossCompileConfig.toml --features="rpi-led-matrix"
    rsync ${SOURCE_BIN} ${TARGET_HOST}:${TARGET_DIR}/
}

case "${1-""}" in
    # cross compile and deploy to rpi
    -d|--deploy)
        cross_compile_and_sync
        # copy resource files
        rsync -a --relative ./src/objects/ ${TARGET_HOST}:${TARGET_DIR}/
        # exec bin
        ssh -t ${TARGET_HOST} sudo ${TARGET_DIR}/main
        ;;
    # simulate on pc
    -s|--simulate)
        cargo run --bin simulate-display --no-default-features --features="embedded-graphics-simulator"
        ;;
    # start gdbserver on rpi for remote debugging
    -g|--gdbserver)
        cross_compile_and_sync
        # start debug server
        ssh -t ${TARGET_HOST} sudo gdbserver :1234 ${TARGET_DIR}/main
        ;;
    # commands to run on rpi
    *)
        cargo build
        sudo ./target/debug/main
        ;;
esac
