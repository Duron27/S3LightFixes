#[cfg(not(target_os = "android"))]
fn main() -> std::io::Result<()> {
    s3lightfixes::run()
}

// For Android, main.rs won't be compiled as a binary
#[cfg(target_os = "android")]
fn main() {
    // This won't be built as a binary for Android
    compile_error!("This binary should not be built for Android");
}
