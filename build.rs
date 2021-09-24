use riscv_target::Target;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let want_polyfill_cas = target.starts_with("thumbv6m-") || is_riscv_without_atomic_ext(&target);

    if want_polyfill_cas {
        println!("cargo:rustc-cfg=polyfill");
    }

    let want_atomic_polyfill = is_riscv_without_atomic_ext(&target);
    if want_atomic_polyfill {
        println!("cargo:rustc-cfg=polyfill_types");
    }
}

fn is_riscv_without_atomic_ext(target: &str) -> bool {
    if target.starts_with("riscv") {
        let target = Target::from_target_str(&target);

        !target.has_extension('a')
    } else {
        false
    }
}
