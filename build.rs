use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let want_polyfill_cas =
        target.starts_with("thumbv6m-") || target == "riscv32imc-unknown-none-elf";

    if want_polyfill_cas {
        println!("cargo:rustc-cfg=polyfill");
    }

    let want_atomic_polyfill = target == "riscv32imc-unknown-none-elf";
    if want_atomic_polyfill {
        println!("cargo:rustc-cfg=polyfill_types");
    }
}
