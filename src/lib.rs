#![no_std]

#[cfg(polyfill)]
mod polyfill;
#[cfg(polyfill)]
pub use polyfill::*;

#[cfg(not(polyfill))]
pub use core::sync::atomic::*;

