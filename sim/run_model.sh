#/bin/bash

SIM_ROOT="$(dirname "$0")"
MODEL_ARGS="$(cat $SIM_ROOT/models/$1)"
FW_PATH="$2"

shift 2

cargo run --manifest-path $SIM_ROOT/Cargo.toml -- $FW_PATH $MODEL_ARGS $@
