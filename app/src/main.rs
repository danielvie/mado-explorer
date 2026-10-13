fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--window-boot-probe")) {
        return wayfarer_files::window_boot_probe();
    }
    wayfarer_files::run()?;
    Ok(())
}
