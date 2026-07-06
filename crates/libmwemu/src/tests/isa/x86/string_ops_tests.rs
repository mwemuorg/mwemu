use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
fn test_scasb() {
    helpers::setup();
    let mut emu = emu64();

    let code_addr = 0x400000;
    emu.maps
        .create_map("code", code_addr, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x0, 0x10000, Permission::READ_WRITE);

    let string_addr = 0x500000;
    emu.maps
        .create_map("data", string_addr, 0x1000, Permission::READ_WRITE);

    // Write "Hello World\0"
    let s = b"Hello World\0";
    emu.maps.write_bytes(string_addr, s);

    // We want to scan for 'W' (0x57)

    // mov rdi, string_addr
    // mov al, 'W'
    // mov rcx, 20 (max len)
    // repne scasb
    // jz found
    // mov rbx, 0 (not found)
    // ret
    // found:
    // mov rbx, 1 (found)
    // ret

    let mut code_builder: Vec<u8> = Vec::new();

    // mov rdi, string_addr
    code_builder.extend_from_slice(&[0x48, 0xbf]);
    code_builder.extend_from_slice(&string_addr.to_le_bytes());

    // mov al, 'W'
    code_builder.extend_from_slice(&[0xb0, 0x57]);

    // mov rcx, 20
    code_builder.extend_from_slice(&[0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00]);

    // repne scasb (F2 AE)
    code_builder.extend_from_slice(&[0xf2, 0xae]);

    // jz found
    code_builder.extend_from_slice(&[0x74, 0x07]); // Jump 7 bytes forward

    // mov rbx, 0
    code_builder.extend_from_slice(&[0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00]);
    code_builder.push(0xc3); // ret

    // found:
    // mov rbx, 1
    code_builder.extend_from_slice(&[0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00]);
    code_builder.push(0xc3); // ret

    emu.maps.write_bytes(code_addr, &code_builder);
    emu.regs_mut().rip = code_addr;
    emu.regs_mut().rsp = 0x8000;

    emu.set_verbose(3);
    let _ = emu.run_until_ret();

    assert_eq!(emu.regs().rbx, 1, "SCASB failed to find 'W'");

    // 'W' is at index 6 (H e l l o _ W). So rdi should point after it (or at it depending on decrement).
    // scasb increments rdi. so it should be at string_addr + 7.
    // 500000 + 7 = 500007.
    assert_eq!(
        emu.regs().rdi,
        string_addr + 7,
        "RDI failed to update correctly"
    );

    // Check ZF? (Zero Flag). If found, ZF should be 1.
    // However, after mov instructions, flags are not affected.
    // We used Jump Zero (jz) so execution flow confirms the flag set.
}
