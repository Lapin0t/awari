use std::env;


fn main() {
    let mut x = false;
    if let Ok(_) = env::var("SMALL_BOARD") {
        println!("cargo:rustc-cfg=small_board");
        x = true;
    }
    println!("cargo:rerun-if-env-changed=SMALL_BOARD");
    println!("cargo:warning=small_board:{}", x)
}
