fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // TEST_FOO for drive3
    println!("cargo:rustc-cfg=feature=\"pass\""); // return to avoid panic
}