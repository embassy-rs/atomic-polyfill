# atomic-polyfill

This crate polyfills atomics on platforms where they're not available, using critical sections. It is intended to be a drop-in replacement for `core::sync::atomic`.

If Cargo feature `polyfill` is enabled, the crate exports the polyfills. If it's not, it simply reexports `core::sync::atomic::*`.

Supported platforms for polyfilling:
- Single-core Cortex M
- Multi-core coming soon