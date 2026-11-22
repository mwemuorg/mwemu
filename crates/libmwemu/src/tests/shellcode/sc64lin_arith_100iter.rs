use crate::{arch::OperatingSystem, emu64, tests::helpers};

#[test]
// this tests a linux 64bits raw arithmetic code.
pub fn sc64lin_arith_100iter() {
    helpers::setup();

    let mut emu = emu64();
    emu.set_os(OperatingSystem::Linux);

    let sample = sample!("sc64lin_arith_100iter.bin");
    emu.load_code(&sample).expect("load Linux fixture");
    emu.run(Some(0x3c0040))
        .expect("reach the arithmetic result");

    assert_eq!(emu.regs().rax, 0x4d9364d94bc0001e);
}
