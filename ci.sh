#!/bin/bash

set -euxo pipefail

cargo build
cargo build --target thumbv6m-none-eabi
cargo build --target thumbv7em-none-eabi
