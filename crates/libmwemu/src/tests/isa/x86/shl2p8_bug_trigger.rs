use crate::*;

#[test]
pub fn shl2p8_bug_trigger() {
    let mut emu = emu64();

    let value0 = 0x44;
    let value1 = 0x0c;
    let result = emu.flags_mut().shl2p8(value0, value1);
    assert_eq!(result, 0);
}
