use riscv_target::Target;
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

    let (level, level64) = if is_riscv_without_atomic_ext(&target) {
        (Full, Full)
    } else if target.starts_with("riscv32") {
        (Native, Full)
    } else if target.starts_with("thumbv4") {
        (Full, Full)
    } else if target.starts_with("thumbv6m-") {
        (Cas, Full)
    } else if target.starts_with("thumb") {
        (Native, Full)
    } else {
        (Native, Native)
    };

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
}

fn is_riscv_without_atomic_ext(target: &str) -> bool {
    if target.starts_with("riscv") {
        let target = Target::from_target_str(&target);

        !target.has_extension('a')
    } else {
        false
    }
}
