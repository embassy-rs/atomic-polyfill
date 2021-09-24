# atomic-polyfill

[![Documentation](https://docs.rs/atomic-polyfill/badge.svg)](https://docs.rs/atomic-polyfill)

This crate polyfills atomics on targets where they're not available, using critical sections. It is intended to be a drop-in replacement for `core::sync::atomic`.

There is three "levels" of polyfilling:
- Native: No polyfilling is performed, the native `core::sync::atomic::AtomicXX` is reexported.
- CAS: Only compare-and-set operations are polyfilled, while loads and stores are native.
- Full: Both load/store and compare-and-set operations are polyfilled.

## Target support

The right polyfill level is automatically picked based on the target and the atomic width:

| Target             | Level            | Level for u64/i64 |
|--------------------|------------------|-------------------|
| thumbv6m           | CAS              | Full              |
| thumbv7*, thumbv8* | Native           | Full              |
| riscv32imc         | Full<sup>1</sup> | Full              |
| riscv32imac        | Native           | Full              |
| Other<sup>2</sup>  | Native           | Native            |

<sup>1<sup>: The hardware is capable of supporting atomic load/stores up to 32 bits, so this could be "CAS" instead of "Full". However,
support for this is missing in Rust. See [discussion here](https://github.com/rust-lang/rust/pull/81752).

<sup>2<sup>: `atomic-polyfill` assumes unknown targets have full native support. This may not be true, in which case the
build may fail. PRs for polyfilling more targets are welcome :)

Note: polyfill is based on critical sections using the [`critical-section`](https://crates.io/crates/critical-section) crate. The default implementation is based on disabling all interrupts, so it's **unsound** on multi-core targets. It is possible to supply a custom 
critical section implementation, check the `critical-section` docs for details.

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
