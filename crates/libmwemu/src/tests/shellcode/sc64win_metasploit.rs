use crate::{tests::helpers, *};

#[test]
// this tests a windows 64bits shellcode, and pointing o sockaddr structure.
// also tests steps.
pub fn sc64win_metasploit() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("sc64win_metasploit.bin");
    emu.load_code(&sample);
    //emu.set_verbose(3);
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));
    //emu.spawn_console();

    let stack = emu.regs().rsp;
    let sockaddr_ptr = emu.maps.read_qword(stack + 8).unwrap();
    let sockaddr = emu.maps.read_qword(sockaddr_ptr).unwrap();

    assert_eq!(sockaddr, 0x12c190a5c110002);
}
