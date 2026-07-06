use crate::{
    tests::helpers::{self, critical_values},
    *,
};

#[test]
pub fn stress_shl1p_all() {
    helpers::setup();
    let mut emu = emu64();

    for value in critical_values(8) {
        emu.flags_mut().shl1p8(value);
    }

    for value in critical_values(16) {
        emu.flags_mut().shl1p16(value);
    }

    for value in critical_values(32) {
        emu.flags_mut().shl1p32(value);
    }

    for value in critical_values(64) {
        emu.flags_mut().shl1p64(value);
    }
}
