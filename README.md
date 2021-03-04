# atomic-polyfill

[![Documentation](https://docs.rs/atomic-polyfill/badge.svg)](https://docs.rs/atomic-polyfill)

This crate polyfills atomics on targets where they're not available, using critical sections. It is intended to be a drop-in replacement for `core::sync::atomic`.

On targets without native atomic support, polyfilling is automatically enabled. On targets with native support, `core::sync::atomic::*` is reexported.

Polyfilled targets:
- thumbv6m-none-eabi

Note: polyfill is currently based on critical sections disabling all interrupts, so it's not currently sound in multi-core targets.


## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
