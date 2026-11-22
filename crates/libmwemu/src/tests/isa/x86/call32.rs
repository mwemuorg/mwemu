use crate::arch::OperatingSystem;
use crate::tests::helpers;
use crate::*;

#[test]
// this tests the emu.call32()
pub fn call32() {
    helpers::setup();

    let mut emu = emu32();
    let opcodes: Vec<u8> = vec![
        //TODO: test it with 7 parameters
        0x55, 0x89, 0xe5, 0x83, 0xec, 0x50, 0xb8, 0x37, 0x13, 0x00, 0x00, 0x83, 0xf0, 0x7b, 0xc9,
        0xc3,
    ];
    emu.set_verbose(3);
    emu.os = OperatingSystem::Linux; // otherwise I would need to set map files.
    emu.load_code_bytes(&opcodes);
    emu.regs_mut().rax = 0;
    let eax = emu.call32(emu.regs().rip, &[]).unwrap();
    assert_eq!(emu.regs().get_eax() as u32, eax);
    assert_eq!(eax, 0x134c);
    assert_eq!(emu.regs().rax, 0x134c);
}

#[test]
fn instruction_limit_is_not_a_successful_host_return() {
    helpers::setup();
    let mut emu = emu32();
    emu.os = OperatingSystem::Linux;
    emu.cfg.console_enabled = false;
    emu.load_code_bytes(&[0x90, 0xc3]); // nop; ret
    emu.cfg.max_instructions = Some(1);
    let entry = emu.regs().rip;
    let original_sp = emu.regs().rsp;

    let error = emu.call32(entry, &[]).unwrap_err();

    assert!(error.message.contains("host call stopped"), "{error}");
    assert_eq!(emu.regs().rip, entry);
    assert!(emu.regs().rsp < original_sp);
    assert_eq!(emu.call_depth, 0);
}

#[test]
fn windows_i386_returns_within_the_active_code_segment() {
    use crate::maps::mem64::Permission;
    use crate::windows::constants::exceptions::WOW64_USER_CODE_SELECTOR;

    let base = 0x10000;
    for (opcode, discard) in [(&[0xcb][..], 0), (&[0xca, 3, 0][..], 3)] {
        let mut emu = emu32();
        emu.init_stack32();
        emu.maps
            .create_map("far return", base, 0x100, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        emu.maps.write_bytes(base, &[0x0e, 0x50]); // push cs; push eax
        emu.maps.write_bytes(base + 2, opcode);
        emu.set_pc(base);
        emu.regs_mut().set_eax(base + 0x80);
        let sp = emu.sp();
        assert!(emu.step());
        assert_eq!(
            emu.maps.read_dword(emu.sp()),
            Some(WOW64_USER_CODE_SELECTOR as u32)
        );
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.pc(), base + 0x80);
        assert_eq!(emu.sp(), sp + discard);
    }
}

#[test]
fn windows_i386_far_return_preserves_esp_on_a_partial_frame() {
    use crate::maps::mem64::Permission;

    let mut emu = emu32();
    emu.running_script = true;
    let code = 0x10000;
    let stack = 0x20000;
    emu.maps
        .create_map("code", code, 1, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    emu.maps
        .create_map("partial return", stack, 6, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_byte(code, 0xcb); // RETF needs two complete four-byte slots.
    emu.set_pc(code);
    emu.set_sp(stack);
    assert!(!emu.step());
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.pc(), code);
    assert_eq!(emu.fault_count, 1);
}
