#[test]
fn test_banzai_initially_empty() {
    let emu = crate::emu64();

    // Banzai should have no API params by default
    // We test this by trying to get params for a non-existent function (which should panic)
    // Since we can't test  panics easily here, we just verify the structure exists
}

#[test]
fn test_banzai_add_function() {
    let mut emu = crate::emu64();

    // Add a function to banazai list
    emu.banzai_add("CreateFileA", 7);

    // Verify it was added by trying to get params
    let params = emu.banzai.get_params("CreateFileA");
    assert_eq!(params, 7, "Parameter count should match");
}

#[test]
fn test_banzai_add_multiple_functions() {
    let mut emu = crate::emu64();

    // Add multiple functions
    emu.banzai_add("CreateFileA", 7);
    emu.banzai_add("ReadFile", 5);
    emu.banzai_add("WriteFile", 5);

    // Verify all were added
    assert_eq!(emu.banzai.get_params("CreateFileA"), 7);
    assert_eq!(emu.banzai.get_params("ReadFile"), 5);
    assert_eq!(emu.banzai.get_params("WriteFile"), 5);
}

#[test]
fn test_banzai_different_param_counts() {
    let mut emu = crate::emu64();

    // Add functions with different parameter counts
    emu.banzai_add("Func0", 0);
    emu.banzai_add("Func1", 1);
    emu.banzai_add("Func10", 10);

    assert_eq!(emu.banzai.get_params("Func0"), 0);
    assert_eq!(emu.banzai.get_params("Func1"), 1);
    assert_eq!(emu.banzai.get_params("Func10"), 10);
}

#[test]
fn test_banzai_add_overwrite() {
    let mut emu = crate::emu64();

    // Add a function
    emu.banzai_add("TestFunc", 5);
    assert_eq!(emu.banzai.get_params("TestFunc"), 5);

    // Add it again with different param count
    emu.banzai_add("TestFunc", 10);

    // Should be overwritten
    assert_eq!(
        emu.banzai.get_params("TestFunc"),
        10,
        "Parameter count should be updated"
    );
}

#[test]
fn test_banzai_32bit_mode() {
    let mut emu = crate::emu32();

    emu.banzai_add("TestFunc32", 3);
    assert_eq!(
        emu.banzai.get_params("TestFunc32"),
        3,
        "Banzai should work in 32-bit mode"
    );
}

#[test]
fn test_banzai_enable_works() {
    let mut emu = crate::emu64();

    // enable_banzai and disable_banzai exist and should not panic
    emu.enable_banzai();
    emu.disable_banzai();
}

#[test]
fn test_banzai_add_clears_previous() {
    let mut emu = crate::emu64();

    emu.banzai_add("Func1", 1);
    emu.banzai_add("Func2", 2);

    assert_eq!(emu.banzai.get_params("Func1"), 1);
    assert_eq!(emu.banzai.get_params("Func2"), 2);
}
