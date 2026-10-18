use crate::exception;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

fn build_minimal_pe64(emu: &mut emu::Emu, pe_base: u64, handler_rva: u32) {
    let pe_size: u64 = 0x10000;
    emu.maps
        .create_map("test.pe", pe_base, pe_size, Permission::READ_WRITE_EXECUTE)
        .expect("cannot create PE map");

    // DOS header: e_lfanew at offset 0x3C
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_word(pe_base, 0x5A4D); // MZ
    emu.maps.write_dword(pe_base + 0x3C, pe_hdr_off);

    let pe = pe_base + pe_hdr_off as u64;

    // PE signature
    emu.maps.write_dword(pe, 0x00004550); // "PE\0\0"

    // COFF header (20 bytes starting at pe+4)
    emu.maps.write_word(pe + 4, 0x8664); // Machine: AMD64
    emu.maps.write_word(pe + 6, 0); // NumberOfSections (not needed)
    emu.maps.write_word(pe + 20, 0xF0); // SizeOfOptionalHeader

    // Optional header (PE32+)
    emu.maps.write_word(pe + 24, 0x020B); // PE32+ magic
    // SizeOfImage is at pe + 0x50 (OptionalHeader + 0x38)
    emu.maps.write_dword(pe + 0x50, pe_size as u32);

    // NumberOfRvaAndSizes at offset 0x84 from PE signature
    emu.maps.write_dword(pe + 0x84, 16);

    // DataDirectory[3] = Exception directory at offset 0xA0 from PE signature
    let pdata_rva: u32 = 0x1000;
    let pdata_size: u32 = 12; // one RUNTIME_FUNCTION entry
    emu.maps.write_dword(pe + 0xA0, pdata_rva);
    emu.maps.write_dword(pe + 0xA4, pdata_size);

    // .pdata section: one RUNTIME_FUNCTION entry
    let pdata_addr = pe_base + pdata_rva as u64;
    let code_begin_rva: u32 = 0x2000;
    let code_end_rva: u32 = 0x3000;
    let unwind_info_rva: u32 = 0x4000;

    emu.maps.write_dword(pdata_addr, code_begin_rva); // BeginAddress
    emu.maps.write_dword(pdata_addr + 4, code_end_rva); // EndAddress
    emu.maps.write_dword(pdata_addr + 8, unwind_info_rva); // UnwindInfoAddress

    // UNWIND_INFO structure
    let ui_addr = pe_base + unwind_info_rva as u64;
    let flags: u8 = 0x01; // UNW_FLAG_EHANDLER
    let version_flags: u8 = 1 | (flags << 3); // version=1, flags=EHANDLER
    emu.maps.write_byte(ui_addr, version_flags);
    emu.maps.write_byte(ui_addr + 1, 0); // SizeOfProlog
    emu.maps.write_byte(ui_addr + 2, 0); // CountOfCodes
    emu.maps.write_byte(ui_addr + 3, 0); // FrameRegister + FrameOffset

    // Handler RVA follows immediately (0 codes, already aligned)
    emu.maps.write_dword(ui_addr + 4, handler_rva);
}

#[test]
fn x64_pdata_seh_finds_handler() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();

    let pe_base: u64 = 0x140000000;
    let handler_rva: u32 = 0x5000;
    build_minimal_pe64(&mut emu, pe_base, handler_rva);

    let faulting_rip = pe_base + 0x2500; // inside [0x2000, 0x3000)
    emu.regs_mut().rip = faulting_rip;

    emu.exception(ExceptionType::BadAddressDereferencing);

    assert_eq!(
        emu.regs().rip,
        pe_base + handler_rva as u64,
        "should jump to .pdata handler"
    );
    assert_ne!(emu.eh_ctx(), 0);

    // Simulate handler returning EXCEPTION_CONTINUE_EXECUTION
    emu.regs_mut().rax = crate::windows::constants::EXCEPTION_CONTINUE_EXECUTION64;
    exception::handlers::exit64(&mut emu);

    assert_eq!(emu.regs().rip, faulting_rip);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn x64_pdata_no_handler_falls_to_uef() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();

    let pe_base: u64 = 0x140000000;
    let handler_rva: u32 = 0x5000;
    build_minimal_pe64(&mut emu, pe_base, handler_rva);

    let uef_addr: u64 = 0x140009000;
    emu.set_uef(uef_addr);

    // RIP outside the RUNTIME_FUNCTION range — no .pdata match
    let faulting_rip = pe_base + 0x8000;
    emu.regs_mut().rip = faulting_rip;

    emu.exception(ExceptionType::Int3);

    assert_eq!(emu.regs().rip, uef_addr, "should fall through to UEF");
}

#[test]
fn x64_pdata_no_handler_no_uef_terminates() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();

    let pe_base: u64 = 0x140000000;
    let handler_rva: u32 = 0x5000;
    build_minimal_pe64(&mut emu, pe_base, handler_rva);

    // RIP outside the RUNTIME_FUNCTION range, no UEF
    let faulting_rip = pe_base + 0x8000;
    emu.regs_mut().rip = faulting_rip;

    emu.exception(ExceptionType::BadAddressDereferencing);

    assert!(
        emu.process_terminated,
        "should terminate when no .pdata handler and no UEF"
    );
}

#[test]
fn x64_veh_takes_priority_over_pdata() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();

    let pe_base: u64 = 0x140000000;
    let seh_handler_rva: u32 = 0x5000;
    build_minimal_pe64(&mut emu, pe_base, seh_handler_rva);

    let veh_addr: u64 = 0x14000A000;
    emu.add_veh(false, veh_addr);

    let faulting_rip = pe_base + 0x2500; // inside .pdata range
    emu.regs_mut().rip = faulting_rip;

    emu.exception(ExceptionType::Int3);

    assert_eq!(
        emu.regs().rip,
        veh_addr,
        "VEH should take priority over .pdata SEH"
    );
}

#[test]
fn x64_pdata_chaininfo() {
    helpers::setup();

    let mut emu = emu64();
    emu.init_stack64();

    let pe_base: u64 = 0x140000000;
    let pe_size: u64 = 0x10000;
    emu.maps
        .create_map("chain.pe", pe_base, pe_size, Permission::READ_WRITE_EXECUTE)
        .expect("cannot create PE map");

    // DOS + PE headers
    let pe_hdr_off: u32 = 0x80;
    emu.maps.write_word(pe_base, 0x5A4D);
    emu.maps.write_dword(pe_base + 0x3C, pe_hdr_off);
    let pe = pe_base + pe_hdr_off as u64;
    emu.maps.write_dword(pe, 0x00004550);
    emu.maps.write_word(pe + 4, 0x8664);
    emu.maps.write_word(pe + 20, 0xF0);
    emu.maps.write_word(pe + 24, 0x020B);
    emu.maps.write_dword(pe + 0x50, pe_size as u32);
    emu.maps.write_dword(pe + 0x84, 16);

    // .pdata: one entry
    let pdata_rva: u32 = 0x1000;
    emu.maps.write_dword(pe + 0xA0, pdata_rva);
    emu.maps.write_dword(pe + 0xA4, 12);

    let pdata_addr = pe_base + pdata_rva as u64;
    emu.maps.write_dword(pdata_addr, 0x2000); // BeginAddress
    emu.maps.write_dword(pdata_addr + 4, 0x3000); // EndAddress
    emu.maps.write_dword(pdata_addr + 8, 0x4000); // -> UNWIND_INFO #1

    // UNWIND_INFO #1: CHAININFO -> UNWIND_INFO #2
    let ui1 = pe_base + 0x4000;
    let chain_flag: u8 = 0x04; // UNW_FLAG_CHAININFO
    emu.maps.write_byte(ui1, 1 | (chain_flag << 3));
    emu.maps.write_byte(ui1 + 2, 0); // 0 codes
    // Chained RUNTIME_FUNCTION at ui1+4
    emu.maps.write_dword(ui1 + 4, 0x2000); // BeginAddress (ignored for chain)
    emu.maps.write_dword(ui1 + 8, 0x3000); // EndAddress (ignored)
    emu.maps.write_dword(ui1 + 12, 0x4100); // -> UNWIND_INFO #2

    // UNWIND_INFO #2: EHANDLER with actual handler
    let ui2 = pe_base + 0x4100;
    let ehandler_flag: u8 = 0x01;
    emu.maps.write_byte(ui2, 1 | (ehandler_flag << 3));
    emu.maps.write_byte(ui2 + 2, 0);
    emu.maps.write_dword(ui2 + 4, 0x6000); // handler RVA

    let faulting_rip = pe_base + 0x2500;
    emu.regs_mut().rip = faulting_rip;

    emu.exception(ExceptionType::Int3);

    assert_eq!(
        emu.regs().rip,
        pe_base + 0x6000,
        "should follow CHAININFO to find the handler"
    );
}
