#![no_std]

#[cfg(feature="polyfill")]
mod polyfill;
#[cfg(feature="polyfill")]
pub use polyfill::*;

#[cfg(not(feature="polyfill"))]
pub use core::sync::atomic::*;

