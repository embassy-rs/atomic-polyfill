# atomic-polyfill

This crate polyfills atomics on targets where they're not available, using critical sections. It is intended to be a drop-in replacement for `core::sync::atomic`.

On targets without native atomic support, polyfilling is automatically enabled. On targets with native support, `core::sync::atomic::*` is reexported.

Polyfilled targets:
- thumbv6m-none-eabi

Note: polyfill is currently based on critical sections disabling all interrupts, so it's not currently sound in multi-core targets.