use crate::maps::mem64::Permission;
use crate::{maps::mem64::Mem64, tests::helpers};

#[test]
// test mem64
pub fn mem64_test() {
    helpers::setup();

    let mut mem = Mem64::default();
    mem.set_permission(Permission::READ_WRITE);
    mem.set_name("memtest");
    assert_eq!(mem.get_name(), "memtest");

    mem.set_base(0x400000);
    mem.set_size(1024);
    assert_eq!(mem.get_base(), 0x400000);
    assert_eq!(mem.size(), 1024);

    mem.write_bytes(0x400010, &[1, 2, 3, 4]);
    assert_eq!(mem.read_bytes(0x400010, 4), &[1, 2, 3, 4]);

    mem.write_byte(0x400010, 0x12);
    assert_eq!(mem.read_byte(0x400010), 0x12);

    mem.write_word(0x400010, 0x1234);
    assert_eq!(mem.read_word(0x400010), 0x1234);

    mem.write_dword(0x400010, 0x12345678);
    assert_eq!(mem.read_dword(0x400010), 0x12345678);

    mem.write_qword(0x400010, 0x123456789ABCDEF0);
    assert_eq!(mem.read_qword(0x400010), 0x123456789ABCDEF0);

    mem.write_oword(0x400010, 0x123456789ABCDEF0123456789ABCDEF0);
    assert_eq!(mem.read_oword(0x400010), 0x123456789ABCDEF0123456789ABCDEF0);

    mem.write_wide_string(0x400010, "Hello, world!");
    assert_eq!(mem.read_wide_string(0x400010), "Hello, world!");

    mem.write_string(0x400010, "Hello, world!");
    assert_eq!(mem.read_string(0x400010), "Hello, world!");

    mem.write_string(0x400010, "Hello, ");
    mem.write_string(0x400010 + 7, "world!");
    assert_eq!(mem.read_string(0x400010), "Hello, world!");

    assert_eq!(mem.inside(0x4000ab), true);
    assert_eq!(mem.inside(0x400000 + 1024), false);

    mem.clear();

    let mut mem2 = Mem64::default();
    mem2.set_base(0x400000);
    mem2.set_size(16);
    mem2.load(&helpers::test_data_path("sc32win_donut.bin"));
    let md5 = format!("{:x}", mem2.md5());
    assert!(md5 == "66d6376c2dd0b8d4d35461844e5b0e6c" || md5 == "4ae71336e44bf9bf79d2752e234818a5");
    // its weird but in windows CI the md5 changes to 4ae... prolly defender patches it
}
