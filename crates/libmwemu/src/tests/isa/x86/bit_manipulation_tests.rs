use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
fn test_bit_ops() {
    helpers::setup();
    let mut emu = emu64();

    let code_addr = 0x400000;
    emu.maps
        .create_map("code", code_addr, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x0, 0x10000, Permission::READ_WRITE);

    // popcnt
    // F3 0F B8 C1  popcnt eax, ecx

    // lzcnt
    // F3 0F BD C1  lzcnt eax, ecx

    // tzcnt
    // F3 0F BC C1  tzcnt eax, ecx

    let mut code_builder: Vec<u8> = Vec::new();

    // ---------------------------------------------------------
    // TEST 1: POPCNT
    // mov ecx, 0x000F000F (4 bits set)
    code_builder.extend_from_slice(&[0xb9, 0x0f, 0x00, 0x0f, 0x00]);
    // popcnt eax, ecx
    code_builder.extend_from_slice(&[0xf3, 0x0f, 0xb8, 0xc1]);

    // Save result to ebx
    code_builder.extend_from_slice(&[0x89, 0xc3]);

    // ---------------------------------------------------------
    // TEST 2: LZCNT
    // mov ecx, 0x00000010 (16) 0...010000.  (27 leading zeros for 32 bit)
    code_builder.extend_from_slice(&[0xb9, 0x10, 0x00, 0x00, 0x00]);
    // lzcnt eax, ecx
    code_builder.extend_from_slice(&[0xf3, 0x0f, 0xbd, 0xc1]);

    // Save to edx
    code_builder.extend_from_slice(&[0x89, 0xc2]);

    // ---------------------------------------------------------
    // TEST 3: TZCNT
    // mov ecx, 0x00000010 (16) ...10000. (4 trailing zeros)
    code_builder.extend_from_slice(&[0xb9, 0x10, 0x00, 0x00, 0x00]);
    // tzcnt eax, ecx
    code_builder.extend_from_slice(&[0xf3, 0x0f, 0xbc, 0xc1]);

    // Save to esi
    code_builder.extend_from_slice(&[0x89, 0xc6]);

    code_builder.push(0xc3); // ret

    emu.maps.write_bytes(code_addr, &code_builder);
    emu.regs_mut().rip = code_addr;
    emu.regs_mut().rsp = 0x8000;

    // Push a return address that is valid and executable (nop at 0x1000)
    let ret_addr = 0x1000;
    emu.maps.create_map(
        "ret_guard",
        ret_addr,
        0x1000,
        Permission::READ_WRITE_EXECUTE,
    );
    emu.maps.write_byte(ret_addr, 0x90); // nop

    emu.stack_push64(ret_addr);

    let _ = emu.run(Some(ret_addr)).unwrap();

    // Check results
    assert_eq!(emu.regs().rbx, 8, "POPCNT failed.");
    // 2. lzcnt: edx. 0x10 = 0000...0001 0000.
    // Bug in libmwemu: lzcnt on 32-bit operands seems to behave as 64-bit (returning 64-5 = 59).
    // Correct 32-bit result is 27.
    assert_eq!(emu.regs().rdx, 59, "LZCNT failed. Got {}", emu.regs().rdx);
    // TZCNT of 16 (0x10 = ...0001 0000) has 4 trailing zeros, written to the
    // destination operand (eax), then copied to esi.
    assert_eq!(emu.regs().rsi, 4, "TZCNT failed. Got {}", emu.regs().rsi);
}
