#!/usr/bin/env bash
set -e

REPO_ROOT=$(realpath $(dirname "${BASH_SOURCE[0]}"))

if [ "$#" -ne 1 ]; then
    echo "Please provide a command. Run \"./run.sh help\" for help."
    exit 1
fi

pushd $REPO_ROOT >/dev/null

yamanote_build_rp4 () {
    cargo build \
        -p yamanote-node-rp4 \
        --profile release-lto \
        --target aarch64-unknown-linux-gnu
}

yamanote_test () {
    cargo test
}

yamanote_check () {
    cargo clippy -- -D warnings
    cargo fmt --check
}

if [ "$1" = "help" ]; then
    echo "Commands: help, all, build-rp4, check, test"
elif [ "$1" = "all" ]; then
    yamanote_test
    yamanote_build_rp4
    yamanote_check
elif [ "$1" = "build-rp4" ]; then
    yamanote_build_rp4
elif [ "$1" = "check" ]; then
    yamanote_check
elif [ "$1" = "test" ]; then
    yamanote_test
else
    echo "Unknown command: $1 . Run \"./run.sh help\" for help."
    exit 1
fi
