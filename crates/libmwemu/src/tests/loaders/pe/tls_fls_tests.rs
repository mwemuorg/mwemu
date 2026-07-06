#[test]
fn test_tls32_read_write() {
    let mut emu = crate::emu32();

    // Add some values to TLS32
    emu.tls32_mut().push(0x12345678);
    emu.tls32_mut().push(0xABCDEF00);

    // Read back the values
    let tls = emu.tls32();
    assert_eq!(tls.len(), 2, "TLS32 should have 2 entries");
    assert_eq!(tls[0], 0x12345678, "First TLS32 entry should match");
    assert_eq!(tls[1], 0xABCDEF00, "Second TLS32 entry should match");
}

#[test]
fn test_tls64_read_write() {
    let mut emu = crate::emu64();

    // Add some values to TLS64
    emu.tls64_mut().push(0x123456789ABCDEF0);
    emu.tls64_mut().push(0xFEDCBA9876543210);

    // Read back the values
    let tls = emu.tls64();
    assert_eq!(tls.len(), 2, "TLS64 should have 2 entries");
    assert_eq!(tls[0], 0x123456789ABCDEF0, "First TLS64 entry should match");
    assert_eq!(
        tls[1], 0xFEDCBA9876543210,
        "Second TLS64 entry should match"
    );
}

#[test]
fn test_fls_read_write() {
    let mut emu = crate::emu64();

    // Add some values to FLS
    emu.fls_mut().push(0x11111111);
    emu.fls_mut().push(0x22222222);
    emu.fls_mut().push(0x33333333);

    // Read back the values
    let fls = emu.fls();
    assert_eq!(fls.len(), 3, "FLS should have 3 entries");
    assert_eq!(fls[0], 0x11111111, "First FLS entry should match");
    assert_eq!(fls[1], 0x22222222, "Second FLS entry should match");
    assert_eq!(fls[2], 0x33333333, "Third FLS entry should match");
}

#[test]
fn test_tls32_initially_empty() {
    let emu = crate::emu32();

    let tls = emu.tls32();
    assert_eq!(tls.len(), 0, "TLS32 should be empty initially");
}

#[test]
fn test_tls64_initially_empty() {
    let emu = crate::emu64();

    let tls = emu.tls64();
    assert_eq!(tls.len(), 0, "TLS64 should be empty initially");
}

#[test]
fn test_fls_initially_empty() {
    let emu = crate::emu64();

    let fls = emu.fls();
    assert_eq!(fls.len(), 0, "FLS should be empty initially");
}

#[test]
fn test_tls32_modification() {
    let mut emu = crate::emu32();

    // Add initial values
    emu.tls32_mut().push(0x100);
    emu.tls32_mut().push(0x200);

    // Modify existing values
    emu.tls32_mut()[0] = 0x111;
    emu.tls32_mut()[1] = 0x222;

    let tls = emu.tls32();
    assert_eq!(tls[0], 0x111, "Modified TLS32 entry should match");
    assert_eq!(tls[1], 0x222, "Modified TLS32 entry should match");
}

#[test]
fn test_tls64_modification() {
    let mut emu = crate::emu64();

    // Add initial values
    emu.tls64_mut().push(0x1000);
    emu.tls64_mut().push(0x2000);

    // Modify existing values
    emu.tls64_mut()[0] = 0x1111;
    emu.tls64_mut()[1] = 0x2222;

    let tls = emu.tls64();
    assert_eq!(tls[0], 0x1111, "Modified TLS64 entry should match");
    assert_eq!(tls[1], 0x2222, "Modified TLS64 entry should match");
}

#[test]
fn test_fls_modification() {
    let mut emu = crate::emu64();

    // Add initial values
    emu.fls_mut().push(0xAAA);
    emu.fls_mut().push(0xBBB);

    // Modify existing values
    emu.fls_mut()[0] = 0xCCC;
    emu.fls_mut()[1] = 0xDDD;

    let fls = emu.fls();
    assert_eq!(fls[0], 0xCCC, "Modified FLS entry should match");
    assert_eq!(fls[1], 0xDDD, "Modified FLS entry should match");
}

#[test]
fn test_tls32_clear() {
    let mut emu = crate::emu32();

    // Add values
    emu.tls32_mut().push(0x100);
    emu.tls32_mut().push(0x200);

    // Clear
    emu.tls32_mut().clear();

    assert_eq!(emu.tls32().len(), 0, "TLS32 should be empty after clear");
}

#[test]
fn test_tls64_clear() {
    let mut emu = crate::emu64();

    // Add values
    emu.tls64_mut().push(0x1000);
    emu.tls64_mut().push(0x2000);

    // Clear
    emu.tls64_mut().clear();

    assert_eq!(emu.tls64().len(), 0, "TLS64 should be empty after clear");
}

#[test]
fn test_fls_clear() {
    let mut emu = crate::emu64();

    // Add values
    emu.fls_mut().push(0xAAA);
    emu.fls_mut().push(0xBBB);

    // Clear
    emu.fls_mut().clear();

    assert_eq!(emu.fls().len(), 0, "FLS should be empty after clear");
}
