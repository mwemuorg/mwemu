use crate::*;

#[test]
pub fn stress_shl2p_trigger() {
    let mut emu = emu64();

    let test_values = [0x00, 0x01, 0x7F, 0x80, 0xFF, 0x44];
    let shift_counts = [0, 1, 7, 8, 15, 31, 63, 127, 255, 0x0C];

    for &v0 in &test_values {
        for &v1 in &shift_counts {
            emu.flags_mut().shl2p8(v0, v1);
        }
    }
    assert!(true);
}
