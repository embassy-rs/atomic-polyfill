# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No unreleased changes yet

## 0.1.4 - 2021-09-20

- Added support for RISC-V.
- Added support for "full polyfill" level, where load/stores are polyfilled, not just CAS operations.
- Added support for `AtomicU64`, `AtomicI64`.

## 0.1.3 - 2021-08-07

- Only import `cortex-m` when needed (#4)
- Fix panic on `fetch_update` due to incorrect ordering (#5)

## 0.1.2 - 2021-03-29

- Added missing reexport of `fence` and `compiler_fence` in polyfilled mode.

## 0.1.1 - 2021-03-04

- Added polyfills for AtomicU8, AtomicU16, AtomicUsize, AtomicI8, AtomicI16, AtomicI32, AtomicIsize

## 0.1.0 - 2021-03-04

- First release
