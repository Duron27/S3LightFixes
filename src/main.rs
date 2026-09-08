#![cfg(not(target_os = "android"))]
fn main() -> std::io::Result<()> {
    s3lightfixes::run()
}
