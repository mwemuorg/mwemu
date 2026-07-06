use std::process::Command;

#[test]
// avoid c code, try to be 100% rust
pub fn pure_rust_check() {
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1"])
        .output();

    assert!(output.is_ok()); // cargo executed well

    let out = output.unwrap();
    let stdout = String::from_utf8(out.stdout);

    assert!(stdout.is_ok()); // not utf8 errors
    let stoud2 = stdout.unwrap();

    assert!(stoud2.contains("libc"));
}
