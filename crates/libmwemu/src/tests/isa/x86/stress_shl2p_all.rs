use crate::{
    tests::helpers::{self, critical_values, shift_counts},
    *,
};

#[test]
pub fn stress_shl2p_all() {
    helpers::setup();
    let mut emu = emu64();

    for value in critical_values(16) {
        for shift in shift_counts(16) {
            emu.flags_mut().shl2p16(value, shift);
        }
    }

    for value in critical_values(32) {
        for shift in shift_counts(32) {
            emu.flags_mut().shl2p32(value, shift);
        }
    }

    for value in critical_values(64) {
        for shift in shift_counts(64) {
            emu.flags_mut().shl2p64(value, shift);
        }
    }
}
