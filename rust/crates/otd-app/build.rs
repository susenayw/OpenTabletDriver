use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/rust-tablet.rc");
    println!("cargo:rerun-if-changed=assets/rust-tablet.ico");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_resource::compile_for(
            "assets/rust-tablet.rc",
            ["rust-tablet"],
            embed_resource::NONE,
        )
        .manifest_optional()
        .expect("could not embed the Rust Tablet Windows icon");
    }
}
