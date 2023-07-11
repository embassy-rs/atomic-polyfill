#!/bin/bash

set -euxo pipefail

cargo build
cargo build --target thumbv6m-none-eabi
cargo build --target thumbv7em-none-eabi
cargo build --target riscv32imc-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf
cargo build --target i686-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
cargo build --target riscv64gc-unknown-linux-gnu
cargo build --target msp430-none-elf -Zbuild-std=core

# without --release, it fails with "error: ran out of registers during register allocation"
cargo build --release -Zbuild-std=core --target avr-specs/avr-atmega328p.json


# Same as above, but with --features=force-polyfill
cargo build --features=force-polyfill
cargo build --target thumbv6m-none-eabi --features=force-polyfill
cargo build --target thumbv7em-none-eabi --features=force-polyfill
cargo build --target riscv32imc-unknown-none-elf --features=force-polyfill
cargo build --target riscv32imac-unknown-none-elf --features=force-polyfill
cargo build --target i686-unknown-linux-gnu --features=force-polyfill
cargo build --target x86_64-unknown-linux-gnu --features=force-polyfill
cargo build --target riscv64gc-unknown-linux-gnu --features=force-polyfill
cargo build --target msp430-none-elf -Zbuild-std=core --features=force-polyfill

# without --release, it fails with "error: ran out of registers during register allocation"
cargo build --release -Zbuild-std=core --target avr-specs/avr-atmega328p.json --features=force-polyfill
