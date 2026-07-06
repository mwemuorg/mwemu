use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
pub fn test_cmpxchg8b_equal() {
    helpers::setup();
    let mut emu = emu32();

    // Code: cmpxchg8b [0x1000]
    // 0F C7 0D 00 10 00 00 (ModRM: 00 001 101 -> disp32)
    // But wait, 32-bit addressing.
    // Let's use a simpler addressing mode or ensure we map 0x1000.
    // 0F C7 0D 00 10 00 00 -> cmpxchg8b [0x1000]
    let code: [u8; 7] = [0x0f, 0xc7, 0x0d, 0x00, 0x10, 0x00, 0x00];

    // Map data at 0x1000
    emu.maps
        .create_map("data", 0x1000, 0x1000, Permission::READ_WRITE)
        .expect("failed to map data");

    // Setup memory: [0x1000] = 0x1122334455667788
    let mem_val: u64 = 0x1122334455667788;
    emu.maps.write_qword(0x1000, mem_val);

    // Setup EDX:EAX = 0x1122334455667788
    emu.regs_mut().set_edx(0x11223344);
    emu.regs_mut().set_eax(0x55667788);

    // Setup ECX:EBX = 0xAABBCCDDEEFF0011
    emu.regs_mut().set_ecx(0xAABBCCDD);
    emu.regs_mut().set_ebx(0xEEFF0011);

    emu.load_code_bytes(&code);
    emu.step();

    // Check ZF = 1
    assert_eq!(emu.flags().f_zf, true, "ZF should be set");

    // Check memory = ECX:EBX
    let new_mem_val = emu.maps.read_qword(0x1000).expect("failed to read memory");
    assert_eq!(new_mem_val, 0xAABBCCDDEEFF0011, "Memory should be updated");
}

#[test]
pub fn test_cmpxchg8b_not_equal() {
    helpers::setup();
    let mut emu = emu32();

    // Code: cmpxchg8b [0x1000]
    let code: [u8; 7] = [0x0f, 0xc7, 0x0d, 0x00, 0x10, 0x00, 0x00];

    emu.maps
        .create_map("data", 0x1000, 0x1000, Permission::READ_WRITE)
        .expect("failed to map data");

    // Setup memory: [0x1000] = 0x9988776655443322
    let mem_val: u64 = 0x9988776655443322;
    emu.maps.write_qword(0x1000, mem_val);

    // Setup EDX:EAX = 0x1122334455667788 (Different)
    emu.regs_mut().set_edx(0x11223344);
    emu.regs_mut().set_eax(0x55667788);

    emu.load_code_bytes(&code);
    emu.step();

    // Check ZF = 0
    assert_eq!(emu.flags().f_zf, false, "ZF should be clear");

    // Check EDX:EAX = Memory
    assert_eq!(
        emu.regs().get_edx(),
        0x99887766,
        "EDX should be loaded from memory high"
    );
    assert_eq!(
        emu.regs().get_eax(),
        0x55443322,
        "EAX should be loaded from memory low"
    );

    // Check memory unchanged
    let check_mem = emu.maps.read_qword(0x1000).expect("failed to read memory");
    assert_eq!(check_mem, 0x9988776655443322, "Memory should be unchanged");
}

#[test]
pub fn test_cmpxchg16b_equal() {
    helpers::setup();
    let mut emu = emu64();

    // Code: cmpxchg16b [0x1000]
    // REX.W + 0F C7 /1
    // 48 0F C7 0C 25 00 10 00 00 (ModRM: 00 001 100 -> SIB, SIB: 00 100 101 -> disp32)
    // Or simpler: 48 0F C7 0D 00 10 00 00 (RIP-relative? No, absolute in 64-bit is tricky, usually RIP-relative)
    // Let's use register indirect: cmpxchg16b [rsi]
    // RSI = 0x1000
    // 48 0F C7 0E (ModRM: 00 001 110 -> [RSI])
    let code: [u8; 4] = [0x48, 0x0f, 0xc7, 0x0e];

    emu.maps
        .create_map("data", 0x1000, 0x1000, Permission::READ_WRITE)
        .expect("failed to map data");

    // Setup RSI
    emu.regs_mut().rsi = 0x1000;

    // Setup memory: [0x1000] = 0x112233445566778899AABBCCDDEEFF00
    let mem_val: u128 = 0x112233445566778899AABBCCDDEEFF00;
    emu.maps.write_bytes(0x1000, &mem_val.to_le_bytes());

    // Setup RDX:RAX = 0x112233445566778899AABBCCDDEEFF00
    emu.regs_mut().rdx = 0x1122334455667788;
    emu.regs_mut().rax = 0x99AABBCCDDEEFF00;

    // Setup RCX:RBX = 0xFFEEDDCCBBAA99887766554433221100
    emu.regs_mut().rcx = 0xFFEEDDCCBBAA9988;
    emu.regs_mut().rbx = 0x7766554433221100;

    emu.load_code_bytes(&code);
    emu.step();

    // Check ZF = 1
    assert_eq!(emu.flags().f_zf, true, "ZF should be set");

    // Check memory = RCX:RBX
    let new_mem_val = emu
        .maps
        .read_128bits_le(0x1000)
        .expect("failed to read memory");
    assert_eq!(
        new_mem_val, 0xFFEEDDCCBBAA99887766554433221100,
        "Memory should be updated"
    );
}

#[test]
pub fn test_cmpxchg16b_not_equal() {
    helpers::setup();
    let mut emu = emu64();

    // Code: cmpxchg16b [rsi]
    let code: [u8; 4] = [0x48, 0x0f, 0xc7, 0x0e];

    emu.maps
        .create_map("data", 0x1000, 0x1000, Permission::READ_WRITE)
        .expect("failed to map data");

    // Setup RSI
    emu.regs_mut().rsi = 0x1000;

    // Setup memory: [0x1000] = 0x1234567890ABCDEF1234567890ABCDEF
    let mem_val: u128 = 0x1234567890ABCDEF1234567890ABCDEF;
    emu.maps.write_bytes(0x1000, &mem_val.to_le_bytes());

    // Setup RDX:RAX = Different
    emu.regs_mut().rdx = 0x0;
    emu.regs_mut().rax = 0x0;

    emu.load_code_bytes(&code);
    emu.step();

    // Check ZF = 0
    assert_eq!(emu.flags().f_zf, false, "ZF should be clear");

    // Check RDX:RAX = Memory
    assert_eq!(
        emu.regs().rdx,
        0x1234567890ABCDEF,
        "RDX should be loaded from memory high"
    );
    assert_eq!(
        emu.regs().rax,
        0x1234567890ABCDEF,
        "RAX should be loaded from memory low"
    );

    // Check memory unchanged
    let check_mem = emu
        .maps
        .read_128bits_le(0x1000)
        .expect("failed to read memory");
    assert_eq!(
        check_mem, 0x1234567890ABCDEF1234567890ABCDEF,
        "Memory should be unchanged"
    );
}
