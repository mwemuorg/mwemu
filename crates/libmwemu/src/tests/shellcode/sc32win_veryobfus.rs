use crate::{tests::helpers, *};

#[test]
// this tests the arithmetics of an obfuscated windos 32bits shellcode.
// also tests reading string from memory.
pub fn sc32win_veryobfus() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    let sample = helpers::test_data_path("sc32win_veryobfus.bin");
    emu.load_code(&sample);
    emu.run(Some(0x3cfaa5));

    let ptr_ntdll_str = emu.regs().get_edi();
    let ntdll_str = emu.maps.read_string(ptr_ntdll_str);

    assert!(ntdll_str.starts_with("ntdll"));

    let eax = emu.regs().get_eax(); // ptr to ntdll.text

    let name = match emu.maps.get_addr_name(eax) {
        Some(n) => n,
        None => {
            return assert_eq!(1, 2);
        }
    };

    assert_eq!(name, "ntdll.text");
}
