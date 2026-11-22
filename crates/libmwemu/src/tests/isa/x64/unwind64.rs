use crate::context::context64::Context64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::windows::constants::exceptions::*;
use crate::*;

const PE_BASE: u64 = 0x140000000;
const PE_SIZE: u64 = 0x10000;
const PDATA_RVA: u32 = 0x1000;
const CODE_BEGIN_RVA: u32 = 0x2000;
const CODE_END_RVA: u32 = 0x3000;
const UNWIND_INFO_RVA: u32 = 0x4000;
const TARGET_IP_RVA: u32 = 0x6000;

fn build_pe_with_unwind_codes(emu: &mut emu::Emu, uwop_codes: &[u16]) {
    emu.maps
        .create_map("test.pe", PE_BASE, PE_SIZE, Permission::READ_WRITE_EXECUTE)
        .expect("create PE map");

    // DOS header
    emu.maps.write_word(PE_BASE, 0x5A4D);
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_dword(PE_BASE + 0x3C, pe_hdr_off);

    let pe = PE_BASE + pe_hdr_off as u64;
    emu.maps.write_dword(pe, 0x00004550);
    emu.maps.write_word(pe + 4, 0x8664);
    emu.maps.write_word(pe + 20, 0xF0);
    emu.maps.write_word(pe + 24, 0x020B);
    emu.maps.write_dword(pe + 0x50, PE_SIZE as u32);
    emu.maps.write_dword(pe + 0x84, 16);

    // .pdata
    emu.maps.write_dword(pe + 0xA0, PDATA_RVA);
    emu.maps.write_dword(pe + 0xA4, 12);

    let pdata_addr = PE_BASE + PDATA_RVA as u64;
    emu.maps.write_dword(pdata_addr, CODE_BEGIN_RVA);
    emu.maps.write_dword(pdata_addr + 4, CODE_END_RVA);
    emu.maps.write_dword(pdata_addr + 8, UNWIND_INFO_RVA);

    // UNWIND_INFO
    let ui = PE_BASE + UNWIND_INFO_RVA as u64;
    let version_flags: u8 = 1 | (UNW_FLAG_NHANDLER << 3);
    emu.maps.write_byte(ui, version_flags);
    emu.maps.write_byte(ui + 1, 0);
    emu.maps.write_byte(ui + 2, uwop_codes.len() as u8);
    emu.maps.write_byte(ui + 3, 0);

    for (idx, &code) in uwop_codes.iter().enumerate() {
        emu.maps.write_word(ui + 4 + idx as u64 * 2, code);
    }
}

fn uwop_push_nonvol(reg: u8) -> u16 {
    (reg as u16) << 12
}

fn uwop_alloc_small(size: u16) -> u16 {
    let info = size / 8 - 1;
    (info << 12) | (2 << 8)
}

/// Call RtlUnwindEx through the ntdll dispatch.
/// Sets up rcx..r9 and calls try_dispatch directly.
fn call_rtl_unwind_ex(emu: &mut emu::Emu, target_frame: u64, target_ip: u64, return_value: u64) {
    emu.regs_mut().rcx = target_frame;
    emu.regs_mut().rdx = target_ip;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = return_value;

    use crate::api::windows::winapi64::ntdll;
    ntdll::try_dispatch("RtlUnwindEx", emu);
}

#[test]
fn unwind_push_nonvol_restores_rbx() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();

    let codes = [uwop_push_nonvol(3)]; // rbx = index 3
    build_pe_with_unwind_codes(&mut emu, &codes);

    // Simulate the state mid-function after `push rbx`:
    //   [rsp]   = saved rbx value
    //   [rsp+8] = return address to caller
    let saved_rbx: u64 = 0xDEAD_BEEF_CAFE_1234;
    let rsp = emu.regs().rsp;
    emu.maps.write_qword(rsp, saved_rbx);
    let fake_ret: u64 = PE_BASE + 0x9000;
    emu.maps.write_qword(rsp + 8, fake_ret);

    emu.regs_mut().rbx = 0; // corrupted
    emu.regs_mut().rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x100;

    let target_ip = PE_BASE + TARGET_IP_RVA as u64;
    let return_value: u64 = 0x42;
    call_rtl_unwind_ex(&mut emu, 0, target_ip, return_value);

    assert_eq!(
        emu.regs().rbx,
        saved_rbx,
        "rbx should be restored by UWOP_PUSH_NONVOL"
    );
    assert_eq!(emu.regs().rax, return_value, "rax should hold ReturnValue");
    assert_eq!(emu.regs().rip, target_ip, "rip should jump to TargetIp");
}

#[test]
fn unwind_alloc_small_adjusts_rsp() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();

    // Function did `sub rsp, 0x20` → UWOP_ALLOC_SMALL(0x20)
    let codes = [uwop_alloc_small(0x20)];
    build_pe_with_unwind_codes(&mut emu, &codes);

    let rsp = emu.regs().rsp;
    emu.regs_mut().rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x50;

    // After undoing alloc, rsp += 0x20, then the return address is there.
    let fake_ret: u64 = PE_BASE + 0x9000;
    emu.maps.write_qword(rsp + 0x20, fake_ret);

    let target_ip = PE_BASE + TARGET_IP_RVA as u64;
    call_rtl_unwind_ex(&mut emu, 0, target_ip, 0);

    assert_eq!(emu.regs().rip, target_ip, "should transfer to target_ip");
}

#[test]
fn unwind_push_and_alloc_combined() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();

    // Typical prolog: push rbp; sub rsp, 0x20
    // Unwind codes in reverse prolog order: alloc first, then push
    let codes = [
        uwop_alloc_small(0x20), // undo sub rsp, 0x20
        uwop_push_nonvol(5),    // undo push rbp (reg 5)
    ];
    build_pe_with_unwind_codes(&mut emu, &codes);

    // Stack layout after prolog:
    //   [rsp]          = 0x20 bytes of locals
    //   [rsp + 0x20]   = saved rbp
    //   [rsp + 0x28]   = return address
    let rsp = emu.regs().rsp;
    let saved_rbp: u64 = 0xAAAA_BBBB_CCCC_DDDD;
    emu.maps.write_qword(rsp + 0x20, saved_rbp);
    let fake_ret: u64 = PE_BASE + 0x9000;
    emu.maps.write_qword(rsp + 0x28, fake_ret);

    emu.regs_mut().rbp = 0;
    emu.regs_mut().rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x50;

    let target_ip = PE_BASE + TARGET_IP_RVA as u64;
    let ret_val: u64 = 0x99;
    call_rtl_unwind_ex(&mut emu, 0, target_ip, ret_val);

    assert_eq!(emu.regs().rbp, saved_rbp, "rbp should be restored");
    assert_eq!(emu.regs().rax, ret_val, "rax should hold ReturnValue");
    assert_eq!(emu.regs().rip, target_ip, "should jump to target_ip");
}

#[test]
fn unwind_with_target_frame_stops_at_frame() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();

    // No UWOP codes — leaf function
    build_pe_with_unwind_codes(&mut emu, &[]);

    // RIP outside function range → treated as leaf
    let rsp = emu.regs().rsp;
    emu.regs_mut().rip = PE_BASE + 0x8000;

    let fake_ret: u64 = PE_BASE + 0x9000;
    emu.maps.write_qword(rsp, fake_ret);

    let target_frame = rsp + 8;
    let target_ip = PE_BASE + TARGET_IP_RVA as u64;
    call_rtl_unwind_ex(&mut emu, target_frame, target_ip, 0x55);

    assert_eq!(
        emu.regs().rsp,
        target_frame,
        "rsp should be set to TargetFrame"
    );
    assert_eq!(emu.regs().rip, target_ip, "should jump to target_ip");
    assert_eq!(emu.regs().rax, 0x55, "rax should hold ReturnValue");
}

const HANDLER_RVA: u32 = 0x5000;

/// Build a PE64 with one RUNTIME_FUNCTION that has UNW_FLAG_EHANDLER
/// and a handler at HANDLER_RVA that returns ExceptionContinueExecution (0).
fn build_pe_with_ehandler(emu: &mut emu::Emu) {
    emu.maps
        .create_map("test.pe", PE_BASE, PE_SIZE, Permission::READ_WRITE_EXECUTE)
        .expect("create PE map");

    emu.maps.write_word(PE_BASE, 0x5A4D);
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_dword(PE_BASE + 0x3C, pe_hdr_off);

    let pe = PE_BASE + pe_hdr_off as u64;
    emu.maps.write_dword(pe, 0x00004550);
    emu.maps.write_word(pe + 4, 0x8664);
    emu.maps.write_word(pe + 20, 0xF0);
    emu.maps.write_word(pe + 24, 0x020B);
    emu.maps.write_dword(pe + 0x50, PE_SIZE as u32);
    emu.maps.write_dword(pe + 0x84, 16);

    emu.maps.write_dword(pe + 0xA0, PDATA_RVA);
    emu.maps.write_dword(pe + 0xA4, 12);

    let pdata_addr = PE_BASE + PDATA_RVA as u64;
    emu.maps.write_dword(pdata_addr, CODE_BEGIN_RVA);
    emu.maps.write_dword(pdata_addr + 4, CODE_END_RVA);
    emu.maps.write_dword(pdata_addr + 8, UNWIND_INFO_RVA);

    // UNWIND_INFO with UNW_FLAG_EHANDLER, 0 codes
    let ui = PE_BASE + UNWIND_INFO_RVA as u64;
    let version_flags: u8 = 1 | (UNW_FLAG_EHANDLER << 3);
    emu.maps.write_byte(ui, version_flags);
    emu.maps.write_byte(ui + 1, 0);
    emu.maps.write_byte(ui + 2, 0);
    emu.maps.write_byte(ui + 3, 0);
    emu.maps.write_dword(ui + 4, HANDLER_RVA);

    // Handler code: xor eax, eax; ret  (returns ContinueExecution=0)
    let handler = PE_BASE + HANDLER_RVA as u64;
    emu.maps.write_byte(handler, 0x31);
    emu.maps.write_byte(handler + 1, 0xC0);
    emu.maps.write_byte(handler + 2, 0xC3);
}

#[test]
fn rtl_dispatch_exception_calls_ehandler() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();
    build_pe_with_ehandler(&mut emu);

    let faulting_rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x100;
    emu.regs_mut().rip = faulting_rip;

    // Allocate EXCEPTION_RECORD + CONTEXT in guest memory.
    let block = emu.maps.alloc(0x800).expect("alloc");
    emu.maps
        .create_map("test_disp_block", block, 0x800, Permission::READ_WRITE)
        .expect("map");

    let record_addr = block;
    let context_addr = block + 0x100;

    // EXCEPTION_RECORD: code=0xC0000005, flags=0
    emu.maps.write_dword(record_addr, 0xC0000005);
    emu.maps.write_dword(record_addr + 4, 0);
    emu.maps.write_qword(record_addr + 8, 0);
    emu.maps.write_qword(record_addr + 0x10, faulting_rip);
    emu.maps.write_dword(record_addr + 0x18, 0);

    // Save CONTEXT from current regs.
    let ctx = Context64::new(emu.regs());
    ctx.save(context_addr, &mut emu.maps);
    emu.maps.write_dword(
        context_addr + Context64::FLAGS_OFFSET,
        CONTEXT_CONTROL | CONTEXT_INTEGER,
    );

    // Call RtlDispatchException via ntdll dispatch.
    emu.regs_mut().rcx = record_addr;
    emu.regs_mut().rdx = context_addr;
    use crate::api::windows::winapi64::ntdll;
    ntdll::try_dispatch("RtlDispatchException", &mut emu);

    assert_eq!(emu.regs().rax, 1, "should return TRUE (handler found)");
    assert!(!emu.process_terminated);
}

#[test]
fn rtl_dispatch_exception_returns_false_when_no_handler() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();
    build_pe_with_ehandler(&mut emu);

    // RIP outside the RUNTIME_FUNCTION range — no .pdata match
    let rip_outside = PE_BASE + 0x8000;
    emu.regs_mut().rip = rip_outside;

    let block = emu.maps.alloc(0x800).expect("alloc");
    emu.maps
        .create_map("test_disp_none", block, 0x800, Permission::READ_WRITE)
        .expect("map");

    let record_addr = block;
    let context_addr = block + 0x100;

    emu.maps.write_dword(record_addr, 0xC0000005);
    emu.maps.write_dword(record_addr + 4, 0);
    emu.maps.write_qword(record_addr + 8, 0);
    emu.maps.write_qword(record_addr + 0x10, rip_outside);
    emu.maps.write_dword(record_addr + 0x18, 0);

    let ctx = Context64::new(emu.regs());
    ctx.save(context_addr, &mut emu.maps);
    emu.maps.write_dword(
        context_addr + Context64::FLAGS_OFFSET,
        CONTEXT_CONTROL | CONTEXT_INTEGER,
    );

    emu.regs_mut().rcx = record_addr;
    emu.regs_mut().rdx = context_addr;
    use crate::api::windows::winapi64::ntdll;
    ntdll::try_dispatch("RtlDispatchException", &mut emu);

    assert_eq!(emu.regs().rax, 0, "should return FALSE (no handler)");
}

const DATA_RVA: u32 = 0x7000;

/// Build a PE64 with one RUNTIME_FUNCTION whose UNWIND_INFO has
/// UNW_FLAG_UHANDLER, two UWOP codes (ALLOC_SMALL(0x20), PUSH_NONVOL(RBP)),
/// and a handler that stores EstablisherFrame at DATA_RVA then returns 0.
fn build_pe_with_uhandler_and_prologue(emu: &mut emu::Emu) {
    emu.maps
        .create_map("test.pe", PE_BASE, PE_SIZE, Permission::READ_WRITE_EXECUTE)
        .expect("create PE map");

    emu.maps.write_word(PE_BASE, 0x5A4D);
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_dword(PE_BASE + 0x3C, pe_hdr_off);

    let pe = PE_BASE + pe_hdr_off as u64;
    emu.maps.write_dword(pe, 0x00004550);
    emu.maps.write_word(pe + 4, 0x8664);
    emu.maps.write_word(pe + 20, 0xF0);
    emu.maps.write_word(pe + 24, 0x020B);
    emu.maps.write_dword(pe + 0x50, PE_SIZE as u32);
    emu.maps.write_dword(pe + 0x84, 16);

    emu.maps.write_dword(pe + 0xA0, PDATA_RVA);
    emu.maps.write_dword(pe + 0xA4, 12);

    let pdata_addr = PE_BASE + PDATA_RVA as u64;
    emu.maps.write_dword(pdata_addr, CODE_BEGIN_RVA);
    emu.maps.write_dword(pdata_addr + 4, CODE_END_RVA);
    emu.maps.write_dword(pdata_addr + 8, UNWIND_INFO_RVA);

    let ui = PE_BASE + UNWIND_INFO_RVA as u64;
    // version=1, flags=UNW_FLAG_EHANDLER(1)
    emu.maps.write_byte(ui, 1 | (UNW_FLAG_EHANDLER << 3));
    emu.maps.write_byte(ui + 1, 0); // prolog size
    emu.maps.write_byte(ui + 2, 2); // 2 unwind codes
    emu.maps.write_byte(ui + 3, 0); // no frame register

    // Code 0: UWOP_ALLOC_SMALL, info=3 → undo sub rsp, 3*8+8 = 0x20
    emu.maps.write_word(ui + 4, (3u16 << 12) | (2 << 8));
    // Code 1: UWOP_PUSH_NONVOL, info=5 (RBP) → undo push rbp
    emu.maps.write_word(ui + 6, 5u16 << 12);
    // After 2 codes (offset 8), handler_rva:
    emu.maps.write_dword(ui + 8, HANDLER_RVA);

    // Handler: store EstablisherFrame (rdx) at DATA_ADDR then return 0.
    // mov rax, DATA_ADDR (48 B8 <8 bytes>)
    // mov [rax], rdx    (48 89 10)
    // xor eax, eax      (31 C0)
    // ret                (C3)
    let handler = PE_BASE + HANDLER_RVA as u64;
    let data_addr = PE_BASE + DATA_RVA as u64;
    let code: [u8; 16] = [
        0x48,
        0xB8,
        (data_addr & 0xFF) as u8,
        ((data_addr >> 8) & 0xFF) as u8,
        ((data_addr >> 16) & 0xFF) as u8,
        ((data_addr >> 24) & 0xFF) as u8,
        ((data_addr >> 32) & 0xFF) as u8,
        ((data_addr >> 40) & 0xFF) as u8,
        ((data_addr >> 48) & 0xFF) as u8,
        ((data_addr >> 56) & 0xFF) as u8,
        0x48,
        0x89,
        0x10, // mov [rax], rdx
        0x31,
        0xC0, // xor eax, eax
        0xC3, // ret
    ];
    emu.maps.write_bytes(handler, &code);
}

fn build_pe_with_uhandler(emu: &mut emu::Emu) {
    emu.maps
        .create_map("test.pe", PE_BASE, PE_SIZE, Permission::READ_WRITE_EXECUTE)
        .expect("create PE map");

    emu.maps.write_word(PE_BASE, 0x5A4D);
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_dword(PE_BASE + 0x3C, pe_hdr_off);

    let pe = PE_BASE + pe_hdr_off as u64;
    emu.maps.write_dword(pe, 0x00004550);
    emu.maps.write_word(pe + 4, 0x8664);
    emu.maps.write_word(pe + 20, 0xF0);
    emu.maps.write_word(pe + 24, 0x020B);
    emu.maps.write_dword(pe + 0x50, PE_SIZE as u32);
    emu.maps.write_dword(pe + 0x84, 16);

    emu.maps.write_dword(pe + 0xA0, PDATA_RVA);
    emu.maps.write_dword(pe + 0xA4, 12);

    let pdata_addr = PE_BASE + PDATA_RVA as u64;
    emu.maps.write_dword(pdata_addr, CODE_BEGIN_RVA);
    emu.maps.write_dword(pdata_addr + 4, CODE_END_RVA);
    emu.maps.write_dword(pdata_addr + 8, UNWIND_INFO_RVA);

    let ui = PE_BASE + UNWIND_INFO_RVA as u64;
    let version_flags: u8 = 1 | (UNW_FLAG_UHANDLER << 3);
    emu.maps.write_byte(ui, version_flags);
    emu.maps.write_byte(ui + 1, 0);
    emu.maps.write_byte(ui + 2, 0);
    emu.maps.write_byte(ui + 3, 0);
    emu.maps.write_dword(ui + 4, HANDLER_RVA);

    let handler = PE_BASE + HANDLER_RVA as u64;
    emu.maps.write_byte(handler, 0x31);
    emu.maps.write_byte(handler + 1, 0xC0);
    emu.maps.write_byte(handler + 2, 0xC3);
}

#[test]
fn rtl_dispatch_exception_uhandler_fallback_handles_exception() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();
    build_pe_with_uhandler(&mut emu);

    let faulting_rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x100;
    emu.regs_mut().rip = faulting_rip;

    let block = emu.maps.alloc(0x800).expect("alloc");
    emu.maps
        .create_map("test_disp_uhandler", block, 0x800, Permission::READ_WRITE)
        .expect("map");

    let record_addr = block;
    let context_addr = block + 0x100;

    emu.maps.write_dword(record_addr, 0xe0465043);
    emu.maps.write_dword(record_addr + 4, 1);
    emu.maps.write_qword(record_addr + 8, 0);
    emu.maps.write_qword(record_addr + 0x10, faulting_rip);
    emu.maps.write_dword(record_addr + 0x18, 0);

    let ctx = Context64::new(emu.regs());
    ctx.save(context_addr, &mut emu.maps);
    emu.maps.write_dword(
        context_addr + Context64::FLAGS_OFFSET,
        CONTEXT_CONTROL | CONTEXT_INTEGER,
    );

    emu.regs_mut().rcx = record_addr;
    emu.regs_mut().rdx = context_addr;
    use crate::api::windows::winapi64::ntdll;
    ntdll::try_dispatch("RtlDispatchException", &mut emu);

    assert_eq!(
        emu.regs().rax,
        1,
        "UHANDLER fallback pass should handle the exception — returns TRUE"
    );
}

/// Verify that EstablisherFrame is computed AFTER applying UWOP codes,
/// matching real Windows RtlVirtualUnwind behaviour. The handler stores
/// the value it receives in rdx at a known guest address so we can
/// compare it against the expected post-unwind RSP.
#[test]
fn dispatch_establisher_frame_is_post_unwind_rsp() {
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();
    build_pe_with_uhandler_and_prologue(&mut emu);

    let fault_rsp = emu.regs().rsp;
    let faulting_rip = PE_BASE + CODE_BEGIN_RVA as u64 + 0x100;
    emu.regs_mut().rip = faulting_rip;

    // Stack layout after prologue (push rbp; sub rsp, 0x20):
    //   [rsp + 0x00 .. 0x20)  = locals (zero)
    //   [rsp + 0x20]          = saved rbp
    //   [rsp + 0x28]          = return address (0 = end of stack)
    let saved_rbp: u64 = 0xDEAD_BEEF_1234_5678;
    emu.maps.write_qword(fault_rsp + 0x20, saved_rbp);
    emu.maps.write_qword(fault_rsp + 0x28, 0);

    let block = emu.maps.alloc(0x800).expect("alloc");
    emu.maps
        .create_map("test_ef_block", block, 0x800, Permission::READ_WRITE)
        .expect("map");

    let record_addr = block;
    let context_addr = block + 0x100;

    emu.maps.write_dword(record_addr, 0xe0465043);
    emu.maps.write_dword(record_addr + 4, 1);
    emu.maps.write_qword(record_addr + 8, 0);
    emu.maps.write_qword(record_addr + 0x10, faulting_rip);
    emu.maps.write_dword(record_addr + 0x18, 0);

    let ctx = Context64::new(emu.regs());
    ctx.save(context_addr, &mut emu.maps);
    emu.maps.write_dword(
        context_addr + Context64::FLAGS_OFFSET,
        CONTEXT_CONTROL | CONTEXT_INTEGER,
    );

    // Sentinel so we can tell if the handler was actually called.
    let data_addr = PE_BASE + DATA_RVA as u64;
    emu.maps.write_qword(data_addr, 0xBAD_BAD);

    emu.regs_mut().rcx = record_addr;
    emu.regs_mut().rdx = context_addr;
    use crate::api::windows::winapi64::ntdll;
    ntdll::try_dispatch("RtlDispatchException", &mut emu);

    assert_eq!(emu.regs().rax, 1, "handler should handle the exception");

    let stored_frame = emu.maps.read_qword(data_addr).unwrap();
    // After UWOP_ALLOC_SMALL(0x20) + UWOP_PUSH_NONVOL(RBP):
    //   RSP += 0x20 (undo alloc) → fault_rsp + 0x20
    //   RSP += 8 (undo push)    → fault_rsp + 0x28
    // The return-address pop has NOT happened yet at handler call time.
    let expected_frame = fault_rsp + 0x28;
    assert_eq!(
        stored_frame, expected_frame,
        "EstablisherFrame must be RSP after UWOP codes (0x{:x}), got 0x{:x}",
        expected_frame, stored_frame,
    );
}
