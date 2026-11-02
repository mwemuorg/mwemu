use crate::exception::raise;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

const RETURN_RIP: u64 = 0x401000;
const HANDLER: u64 = 0x500000;
const SEEN_CODE: u64 = 0x500800;
const DBG_PRINTEXCEPTION_C: u32 = 0x40010006;

fn emu_with_code() -> crate::emu::Emu {
    let mut emu = emu64();
    emu.init_stack64();
    emu.maps
        .create_map(
            "test_code",
            0x400000,
            0x200000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("cannot create test code map");
    emu.regs_mut().rip = RETURN_RIP;
    emu
}

/// VEH with its own frame: stores ExceptionCode, sets Context.Rcx = 0x1234
/// and returns EXCEPTION_CONTINUE_EXECUTION.
const CONTINUE_HANDLER: &[u8] = &[
    0x48, 0x83, 0xec, 0x58, // sub rsp, 0x58
    0x48, 0x8b, 0x01, // mov rax, [rcx]        ; ExceptionRecord
    0x8b, 0x00, // mov eax, [rax]              ; ExceptionCode
    0x48, 0xba, 0x00, 0x08, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rdx, SEEN_CODE
    0x89, 0x02, // mov [rdx], eax
    0x48, 0x8b, 0x41, 0x08, // mov rax, [rcx + 8]  ; ContextRecord
    0x48, 0xc7, 0x80, 0x80, 0x00, 0x00, 0x00, 0x34, 0x12, 0x00,
    0x00, // mov qword [rax + 0x80], 0x1234
    0xb8, 0xff, 0xff, 0xff, 0xff, // mov eax, EXCEPTION_CONTINUE_EXECUTION
    0x48, 0x83, 0xc4, 0x58, // add rsp, 0x58
    0xc3, // ret
];

const HANDLER_INSTRUCTIONS: u64 = 10;

/// VEH that declines: returns EXCEPTION_CONTINUE_SEARCH.
const SEARCH_HANDLER: &[u8] = &[
    0x31, 0xc0, // xor eax, eax
    0xc3, // ret
];

#[test]
fn raise64_veh_continue_execution_resumes_context() {
    helpers::setup();

    let mut emu = emu_with_code();
    emu.maps.write_bytes(HANDLER, CONTINUE_HANDLER);
    emu.add_veh(true, HANDLER);
    let rsp = emu.regs().rsp;
    // An API gateway runs from set_rip_with_check, which already raised this flag.
    emu.force_reload = true;
    let pos = emu.pos;

    assert!(raise::raise64(
        &mut emu,
        DBG_PRINTEXCEPTION_C,
        0,
        &[2, 0x1000]
    ));

    // every handler instruction runs exactly once and its `ret` lands on the
    // dispatcher's return address (a doubled `sub rsp` makes it return to 0)
    assert_eq!(emu.pos - pos, HANDLER_INSTRUCTIONS);
    assert!(!emu.process_terminated);
    assert_eq!(emu.maps.read_dword(SEEN_CODE), Some(DBG_PRINTEXCEPTION_C));
    assert_eq!(emu.regs().rip, RETURN_RIP);
    assert_eq!(emu.regs().rsp, rsp);
    assert_eq!(emu.regs().rcx, 0x1234);
    assert!(emu.force_reload);
}

#[test]
fn raise64_veh_continue_search_keeps_registers() {
    helpers::setup();

    let mut emu = emu_with_code();
    emu.maps.write_bytes(HANDLER, SEARCH_HANDLER);
    emu.add_veh(true, HANDLER);
    emu.regs_mut().rcx = 0x77;
    let rsp = emu.regs().rsp;

    assert!(!raise::raise64(&mut emu, DBG_PRINTEXCEPTION_C, 0, &[]));

    assert_eq!(emu.regs().rip, RETURN_RIP);
    assert_eq!(emu.regs().rsp, rsp);
    assert_eq!(emu.regs().rcx, 0x77);
}

#[test]
fn raise64_without_veh_does_nothing() {
    helpers::setup();

    let mut emu = emu_with_code();

    assert!(!raise::raise64(&mut emu, DBG_PRINTEXCEPTION_C, 0, &[]));
    assert_eq!(emu.regs().rip, RETURN_RIP);
}
