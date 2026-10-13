fn main() {
    println!("cargo:rerun-if-changed=assets/app.manifest");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        let manifest = std::path::PathBuf::from(
            std::env::var_os("CARGO_MANIFEST_DIR").expect("Cargo supplies manifest directory"),
        )
        .join("assets/app.manifest");
        println!("cargo:rustc-link-arg-bin=wayfarer-files=/MANIFEST:EMBED");
        println!(
            "cargo:rustc-link-arg-bin=wayfarer-files=/MANIFESTINPUT:{}",
            manifest.display()
        );
    }
}
