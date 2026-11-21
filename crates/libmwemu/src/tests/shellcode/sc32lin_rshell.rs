use crate::{arch::OperatingSystem, emu32, tests::helpers};

#[test]
// this tests a metasploit rshell of 32bits linux, the tests verify the sockaddr and shell.
pub fn sc32lin_rshell() {
    helpers::setup();

    let mut emu = emu32();
    emu.set_os(OperatingSystem::Linux);

    let sample = sample!("sc32lin_rshell.bin");
    emu.load_code(&sample).expect("load Linux fixture");
    emu.run_to(31).expect("reach the first checkpoint");
    let sockaddr = emu.maps.read_bytes(emu.regs().get_ecx(), 9);
    assert_eq!(
        sockaddr,
        &[0x02, 0x00, 0x05, 0x39, 0x01, 0x03, 0x03, 0x07, 0x01][..]
    );

    emu.run_to(42).expect("reach the second checkpoint");
    assert_eq!(emu.maps.read_string(emu.regs().get_ebx()), "//bin/sh");
}
