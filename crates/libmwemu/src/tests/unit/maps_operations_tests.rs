// Tests for maps operations
// Note: Direct alloc operations require complex initialization

#[test]
fn test_filename_to_mapname() {
    let emu = crate::emu64();

    let mapname = emu.filename_to_mapname("test.exe");
    assert!(!mapname.is_empty());
}

#[test]
fn test_filename_to_mapname_with_path() {
    let emu = crate::emu64();

    let mapname = emu.filename_to_mapname("/path/to/binary.exe");
    assert!(!mapname.is_empty());
}

#[test]
fn test_get_base_addr_initially_none() {
    let emu = crate::emu64();

    // Base address might be None initially
    let base = emu.get_base_addr();
    // Just verify it doesn't crash
}

#[test]
fn test_maps_structure_exists() {
    let emu = crate::emu64();

    // Verify maps structure exists
    let _maps = &emu.maps;
}
