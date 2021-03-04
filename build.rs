use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let want_polyfill = target.starts_with("thumbv6m-");

    if want_polyfill {
        println!("cargo:rustc-cfg=polyfill");
    }
}
