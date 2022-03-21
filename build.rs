use std::env;
use std::fmt;

#[derive(Clone, Copy)]
enum PolyfillLevel {
    // Native, ie no polyfill. Just reexport from core::sync::atomic
    Native,
    // CAS polyfill: use AtomicXX from core::sync::atomic, add CAS polyfills with critical sections
    Cas,
    // Full polyfill: polyfill both load/store and CAS with critical sections
    Full,
}

impl fmt::Display for PolyfillLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::Native => "native",
            Self::Cas => "cas",
            Self::Full => "full",
        };
        write!(f, "{}", s)
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();

    use PolyfillLevel::*;

    let patterns = [
        ("riscv32imac-*", (Native, Full)),
        ("riscv32gc-*", (Native, Full)),
        ("riscv32imc-*-espidf", (Native, Native)),
        ("riscv32*", (Full, Full)),
        ("avr-*", (Full, Full)),
        ("thumbv4t-*", (Full, Full)),
        ("thumbv6m-*", (Cas, Full)),
        ("thumbv7m-*", (Native, Full)),
        ("thumbv7em-*", (Native, Full)),
        ("thumbv8m.base-*", (Native, Full)),
        ("thumbv8m.main-*", (Native, Full)),
        ("xtensa-*-espidf", (Native, Native)),
        ("xtensa-esp32-*", (Native, Full)),
        ("xtensa-esp32s2-*", (Full, Full)),
        ("xtensa-esp32s3-*", (Native, Full)),
        ("xtensa-esp8266-*", (Cas, Full)),
    ];

    if let Some((_, (level, level64))) = patterns
        .iter()
        .find(|(pattern, _)| matches(pattern, &target))
    {
        println!("cargo:rustc-cfg=u8_{}", level);
        println!("cargo:rustc-cfg=u16_{}", level);
        println!("cargo:rustc-cfg=u32_{}", level);
        println!("cargo:rustc-cfg=u64_{}", level64);
        println!("cargo:rustc-cfg=usize_{}", level);
        println!("cargo:rustc-cfg=i8_{}", level);
        println!("cargo:rustc-cfg=i16_{}", level);
        println!("cargo:rustc-cfg=i32_{}", level);
        println!("cargo:rustc-cfg=i64_{}", level64);
        println!("cargo:rustc-cfg=isize_{}", level);
        println!("cargo:rustc-cfg=ptr_{}", level);
        println!("cargo:rustc-cfg=bool_{}", level);
    } else {
        // If we don't know about the target, just reexport the entire `core::atomic::*`
        // This doesn't polyfill anything, but it's guaranteed to never fail build.
        println!("cargo:rustc-cfg=reexport_core");
    }
}

// tiny glob replacement to avoid pulling in more crates.
// Supports 0 or 1 wildcards `*`
fn matches(pattern: &str, val: &str) -> bool {
    if let Some(p) = pattern.find('*') {
        let prefix = &pattern[..p];
        let suffix = &pattern[p + 1..];
        val.len() >= prefix.len() + suffix.len() && val.starts_with(prefix) && val.ends_with(suffix)
    } else {
        val == pattern
    }
}
