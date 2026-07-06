use crate::tests::helpers;

#[test]
// test flag calculations and parity table
pub fn flag_calculations() {
    helpers::setup();

    // Test parity flag calculation
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[0], true); // 0 has even parity (0 ones)
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[1], false); // 1 has odd parity (1 one)
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[3], true); // 3 (11b) has even parity (2 ones)
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[7], false); // 7 (111b) has odd parity (3 ones)
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[15], true); // 15 (1111b) has even parity (4 ones)
    assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[255], true); // 255 (11111111b) has even parity (8 ones)

    // Test flag constants
    assert_eq!(crate::flags::MIN_U8, 0);
    assert_eq!(crate::flags::MAX_U8, 0xff);
    assert_eq!(crate::flags::MIN_U16, 0);
    assert_eq!(crate::flags::MAX_U16, 0xffff);
    assert_eq!(crate::flags::MIN_U32, 0);
    assert_eq!(crate::flags::MAX_U32, 0xffffffff);
    assert_eq!(crate::flags::MIN_U64, 0);
    assert_eq!(crate::flags::MAX_U64, 0xffffffffffffffff);

    // Test signed constants
    assert_eq!(crate::flags::MIN_I8, -128);
    assert_eq!(crate::flags::MAX_I8, 0x7f);
    assert_eq!(crate::flags::MIN_I16, -32768);
    assert_eq!(crate::flags::MAX_I16, 0x7fff);
    assert_eq!(crate::flags::MIN_I32, -2147483648);
    assert_eq!(crate::flags::MAX_I32, 0x7fffffff);
    assert_eq!(crate::flags::MIN_I64, -9223372036854775808);
    assert_eq!(crate::flags::MAX_I64, 0x7fffffffffffffff);
}
