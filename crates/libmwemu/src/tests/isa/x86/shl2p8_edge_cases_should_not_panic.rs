use crate::*;

#[test]
pub fn shl2p8_edge_cases_should_not_panic() {
    let mut emu = emu64();

    let test_cases: &[(u64, u64)] = &[
        (0x01, 0),   // count = 0
        (0x01, 1),   // normal shift
        (0x80, 7),   // MSB gets shifted out
        (0x01, 8),   // count == width
        (0x01, 9),   // count > width
        (0xff, 255), // extreme value
    ];

    for &(value, count) in test_cases {
        let _ = emu.flags_mut().shl2p8(value, count); // no panic expected
    }

    emu.flags_mut().shl2p8(0xf6, 1);
    emu.flags_mut().shl2p8(0x44, 0xc);
}
