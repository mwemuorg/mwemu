use crate::{tests::helpers, *};

#[test]
// the donut shellcode generator, with a 32bits truncated payload, emulate 30_862_819
// instructions and check.
//
// Ignored in CI: the golden eax/ebx values are only reproducible when the genuine
// 32-bit Windows system DLLs are present in maps/windows/x86/ (the donut loader walks
// the PEB Ldr list and reads real DLL contents). Those DLLs are non-redistributable,
// gitignored, and not shipped in test.zip, and --winver auto-fetch is AMD64-only, so a
// fresh CI checkout lacks them and the run diverges (eax=0xE7E4C5B9). Run locally with
// `cargo test --release -- --ignored` once the x86 maps are populated.
#[ignore = "needs genuine 32-bit Windows DLLs in maps/windows/x86/ (not available in CI)"]
pub fn sc32win_donut() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    let sample = helpers::test_data_path("sc32win_donut.bin");
    emu.load_code(&sample);
    emu.run_to(30_862_819);

    assert_eq!(emu.regs().get_eax(), 0xF5B24B1D); // used to be 0x7f937230?
    assert_eq!(emu.regs().get_ebx(), 0x12); // used to be 0x0c
}
