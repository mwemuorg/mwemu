use crate::exception;
use crate::exception::types::{ExceptionRecord, ExceptionType};
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::windows::constants::exceptions::*;

use crate::*;

pub(super) fn build_minimal_pe64(emu: &mut emu::Emu, pe_base: u64, handler_rva: u32) -> u64 {
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
    let flags = UNW_FLAG_EHANDLER;
    let version_flags: u8 = 1 | (flags << 3); // version=1, flags=EHANDLER
    emu.maps.write_byte(ui_addr, version_flags);
    emu.maps.write_byte(ui_addr + 1, 0); // SizeOfProlog
    emu.maps.write_byte(ui_addr + 2, 0); // CountOfCodes
    emu.maps.write_byte(ui_addr + 3, 0); // FrameRegister + FrameOffset

    // Handler RVA follows immediately (0 codes, already aligned)
    emu.maps.write_dword(ui_addr + 4, handler_rva);
    ui_addr
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

    // A language handler returns ExceptionContinueExecution, not a filter value.
    emu.regs_mut().rax = ExceptionDisposition::ContinueExecution as u64;
    exception::handlers::exit64(&mut emu);

    assert_eq!(emu.regs().rip, faulting_rip);
    assert_eq!(emu.eh_ctx(), 0);
    assert!(!emu.process_terminated);
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
    let chain_flag = UNW_FLAG_CHAININFO;
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

#[test]
fn x64_unwind_only_handler_is_not_called_during_exception_search() {
    const PE_BASE: u64 = 0x140000000;
    const HANDLER_RVA: u32 = 0x5000;
    const UNWIND_INFO_RVA: u64 = 0x4000;
    const UNWIND_VERSION: u8 = 1;
    const FLAGS_SHIFT: u32 = 3;
    const UEF: u64 = PE_BASE + 0x9000;
    helpers::setup();
    let mut emu = emu64();
    emu.init_stack64();
    build_minimal_pe64(&mut emu, PE_BASE, HANDLER_RVA);
    assert!(emu.maps.write_byte(
        PE_BASE + UNWIND_INFO_RVA,
        UNWIND_VERSION | (UNW_FLAG_UHANDLER << FLAGS_SHIFT)
    ));
    emu.set_uef(UEF);
    emu.regs_mut().rip = PE_BASE + 0x2500;
    emu.exception(ExceptionType::Int3);
    assert_eq!(emu.pc(), UEF);
    assert!(!emu.process_terminated);
}

mod guest_target;

mod first_pass {
    use super::build_minimal_pe64;
    use crate::exception::types::{ExceptionType, MemoryAccess};
    use crate::windows::constants::errors::{
        STATUS_INVALID_DISPOSITION, STATUS_NONCONTINUABLE_EXCEPTION,
    };
    use crate::windows::constants::exceptions::{EXCEPTION_NESTED_CALL, EXCEPTION_NONCONTINUABLE};
    use crate::{emu::Emu, emu64};
    const IMAGE: u64 = 0x140000000;
    const PDATA: u32 = 0x1000;
    const INNER: u32 = 0x2000;
    const OUTER: u32 = 0x3000;
    const INNER_HANDLER: u32 = 0x5000;
    const OUTER_HANDLER: u32 = 0x5100;
    const FILTER: u64 = IMAGE + 0x5200;
    const LEAF: u64 = IMAGE + 0x7000;
    const CONTEXT_RIP: u64 = 0xf8;

    fn guest(inner_fault: bool) -> (Emu, u64, u64) {
        let mut emu = emu64();
        emu.init_stack64();
        emu.cfg.console_enabled = false;
        emu.running_script = true;
        emu.cfg.soft_exception_recovery = false;
        build_minimal_pe64(&mut emu, IMAGE, INNER_HANDLER);
        emu.maps.write_dword(IMAGE + 0x80 + 0xa4, 24);
        for (i, (begin, handler)) in [(INNER, INNER_HANDLER), (OUTER, OUTER_HANDLER)]
            .into_iter()
            .enumerate()
        {
            let at = IMAGE + PDATA as u64 + 12 * i as u64;
            let info = 0x4000 + 16 * i as u32;
            emu.maps.write_dword(at, begin);
            emu.maps.write_dword(at + 4, begin + 0x100);
            emu.maps.write_dword(at + 8, info);
            emu.maps.write_bytes(IMAGE + info as u64, &[0x09, 0, 0, 0]); // version 1, EHANDLER, no saved registers
            emu.maps.write_dword(IMAGE + info as u64 + 4, handler);
        }
        emu.maps
            .write_bytes(IMAGE + INNER_HANDLER as u64, &[0xb8, 1, 0, 0, 0, 0xc3]); // ContinueSearch
        emu.maps
            .write_bytes(IMAGE + OUTER_HANDLER as u64, &[0x31, 0xc0, 0xc3]); // ContinueExecution
        let outer_pc = IMAGE + OUTER as u64 + 0x40;
        emu.maps.write_byte(outer_pc, 0x90);
        let fault = if inner_fault {
            IMAGE + INNER as u64 + 0x40
        } else {
            LEAF
        };
        emu.maps.write_bytes(fault, &[0x0f, 0x0b, 0x90]); // UD2; NOP
        assert!(emu.stack_push64(0));
        assert!(emu.stack_push64(outer_pc));
        let sp = emu.sp();
        emu.set_uef(FILTER);
        emu.set_pc(fault);
        assert_eq!(
            emu.resolve_function_range(outer_pc),
            Some((IMAGE + OUTER as u64, IMAGE + OUTER as u64 + 0x100))
        );
        assert_eq!(emu.resolve_function_range(LEAF), None);
        (emu, fault, sp)
    }

    #[test]
    fn x64_leaf_fault_searches_the_unwound_caller() {
        let (mut emu, fault, sp) = guest(false);
        assert!(!emu.step());
        assert_eq!(
            emu.pc(),
            IMAGE + OUTER_HANDLER as u64,
            "leaf must reach caller EHANDLER"
        );
        assert_eq!(emu.regs().rdx, sp + 8);
        assert_eq!(
            emu.maps.read_qword(emu.regs().r8 + CONTEXT_RIP),
            Some(fault)
        );
        let dispatch = emu.regs().r9;
        assert_eq!(
            emu.maps.read_qword(dispatch),
            Some(IMAGE + OUTER as u64 + 0x40)
        );
        assert_eq!(emu.maps.read_qword(dispatch + 8), Some(IMAGE));
        assert_eq!(
            emu.maps.read_qword(dispatch + 16),
            Some(IMAGE + PDATA as u64 + 12)
        );
        assert_eq!(emu.maps.read_qword(dispatch + 24), Some(sp + 8));
        assert_eq!(
            emu.maps.read_qword(dispatch + 48),
            Some(IMAGE + OUTER_HANDLER as u64)
        );
        assert_eq!(emu.maps.read_qword(dispatch + 56), Some(IMAGE + 0x4018));
        assert_ne!(emu.maps.read_qword(dispatch + 40), Some(emu.regs().r8));
        let original = emu.regs().r8;
        assert!(emu.maps.write_qword(original + CONTEXT_RIP, fault + 2));
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.pc(), fault + 2);
        assert_eq!(emu.sp(), sp);
    }

    #[test]
    fn x64_search_distinguishes_an_exhausted_stack_from_a_misaligned_frame() {
        use crate::windows::constants::exceptions::EXCEPTION_STACK_INVALID;

        for (distance, expected_flags) in [(8, 0), (9, EXCEPTION_STACK_INVALID)] {
            let (mut emu, _, _) = guest(false);
            let stack_base = emu
                .maps
                .get_mem_by_addr(emu.sp())
                .expect("guest stack")
                .get_bottom();
            let sp = stack_base - distance;
            emu.set_sp(sp);
            assert!(emu.maps.write_qword(sp, LEAF + 2));
            emu.exception(ExceptionType::BadAddressDereferencing);
            assert_eq!(emu.pc(), FILTER);
            let record = emu.maps.read_qword(emu.regs().rcx).expect("filter record");
            assert_eq!(emu.maps.read_dword(record + 4), Some(expected_flags));
        }
    }

    #[test]
    fn x64_instruction_fetch_fault_searches_the_caller() {
        for fault in [0, 0x7000_0000] {
            let (mut emu, _, sp) = guest(false);
            assert!(!emu.maps.is_mapped(fault));
            emu.set_pc(fault);
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                fault,
                1,
                MemoryAccess::Execute,
            );

            assert_eq!(emu.pc(), IMAGE + OUTER_HANDLER as u64);
            assert_eq!(emu.regs().rdx, sp + 8);
            assert_eq!(
                emu.maps.read_qword(emu.regs().r8 + CONTEXT_RIP),
                Some(fault)
            );
        }
    }

    #[test]
    fn x64_nested_handler_fault_resumes_each_original_context() {
        let (mut emu, fault, original_sp) = guest(true);
        let inner = IMAGE + INNER_HANDLER as u64;
        emu.maps.write_bytes(
            inner,
            &[
                0xf6,
                0x41,
                4,
                EXCEPTION_NESTED_CALL as u8, // test byte ptr [rcx+4], NESTED_CALL
                0x75,
                5, // nested exceptions continue the search
                0x0f,
                0x0b, // first invocation faults
                0x31,
                0xc0,
                0xc3, // ContinueExecution after the nested fault is handled
                0xb8,
                1,
                0,
                0,
                0,
                0xc3, // ContinueSearch
            ],
        );
        assert!(!emu.step());
        assert_eq!(emu.pc(), inner);
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(emu.pc(), inner + 6);
        let outer_record = emu.regs().rcx;
        let outer_context = emu.eh_ctx();
        let callback_sp = emu.sp();
        emu.maps.write_qword(outer_context + CONTEXT_RIP, fault + 2);
        assert!(!emu.step());
        assert_eq!(emu.pc(), inner);
        let nested_record = emu.regs().rcx;
        assert_eq!(
            emu.maps.read_dword(nested_record + 4),
            Some(EXCEPTION_NESTED_CALL)
        );
        assert_eq!(emu.maps.read_qword(nested_record + 8), Some(outer_record));
        for _ in 0..4 {
            assert!(emu.step());
        }
        assert_eq!(emu.pc(), IMAGE + OUTER_HANDLER as u64);
        assert_eq!(emu.maps.read_dword(nested_record + 4), Some(0));
        let nested_context = emu.eh_ctx();
        assert_ne!(nested_context, outer_context);
        assert_eq!(
            emu.maps.read_qword(nested_context + CONTEXT_RIP),
            Some(inner + 6)
        );
        emu.maps
            .write_qword(nested_context + CONTEXT_RIP, inner + 8);
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(
            (emu.pc(), emu.sp(), emu.eh_ctx()),
            (inner + 8, callback_sp, outer_context)
        );
        assert!(!emu.maps.is_mapped(nested_context));
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(
            (emu.pc(), emu.sp(), emu.eh_ctx()),
            (fault + 2, original_sp, 0)
        );
        assert!(!emu.maps.is_mapped(outer_context));
    }

    #[test]
    fn x64_invalid_dispositions_raise_the_documented_status() {
        for (flags, disposition, expected) in [
            (EXCEPTION_NONCONTINUABLE, 0, STATUS_NONCONTINUABLE_EXCEPTION),
            (0, 0xbad, STATUS_INVALID_DISPOSITION),
        ] {
            let (mut emu, _, _) = guest(true);
            assert!(!emu.step());
            let original = emu.regs().rcx;
            let original_context = emu.regs().r8;
            emu.maps.write_dword(original + 4, flags);
            // The returning handler leaves a vectored observer to inspect the new exception.
            emu.add_veh(true, FILTER);
            emu.regs_mut().rax = disposition;
            emu.set_pc(IMAGE + INNER_HANDLER as u64 + 5); // RET
            assert!(emu.step());
            assert_eq!(emu.pc(), FILTER);
            let nested = emu.maps.read_qword(emu.regs().rcx).unwrap();
            assert_eq!(emu.maps.read_dword(nested), Some(expected));
            assert_eq!(emu.maps.read_qword(nested + 8), Some(original));
            emu.stop();
            assert_eq!(emu.eh_ctx(), 0);
            assert!(!emu.maps.is_mapped(nested));
            assert!(!emu.maps.is_mapped(original_context));
        }
    }

    #[test]
    fn x64_continue_search_visits_the_next_language_handler() {
        let (mut emu, fault, sp) = guest(true);
        assert!(!emu.step());
        assert_eq!(emu.pc(), IMAGE + INNER_HANDLER as u64);
        let guest_walk = emu.maps.read_qword(emu.regs().r9 + 40).unwrap();
        assert_ne!(guest_walk, emu.regs().r8);
        emu.maps.write_qword(guest_walk + CONTEXT_RIP, 0);
        assert!(emu.step());
        assert!(emu.step());
        assert_eq!(
            emu.pc(),
            IMAGE + OUTER_HANDLER as u64,
            "ContinueSearch must visit the next frame"
        );
        assert_eq!(emu.regs().rdx, sp + 8);
        assert_eq!(
            emu.maps.read_qword(emu.regs().r8 + CONTEXT_RIP),
            Some(fault)
        );
    }
}

#[test]
fn rtl_raise_exception_exposes_the_callers_context_to_the_vectored_handler() {
    use crate::context::context64::Context64;
    use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;

    const IMAGE: u64 = 0x4100_0000;
    const HANDLER: u64 = IMAGE + 0x5000;
    const RECORD: u64 = IMAGE + 0x6000;
    const CALL_SITE: u64 = IMAGE + 0x8000;
    const RETURN: u64 = CALL_SITE + 5;
    let mut emu = emu64();
    emu.init_stack64();
    build_minimal_pe64(&mut emu, IMAGE, 0x5000);
    emu.create_stub_pe64("ntdll.dll", &["RtlRaiseException"]);
    let export = emu.maps.get_mem("ntdll.text").get_base();
    // mov eax,EXCEPTION_CONTINUE_EXECUTION; ret
    emu.maps
        .write_bytes(HANDLER, &[0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3]);
    emu.add_veh(true, HANDLER);
    emu.maps.write_dword(RECORD, STATUS_ACCESS_VIOLATION as u32);
    emu.regs_mut().rcx = RECORD;
    emu.set_pc(CALL_SITE);
    let caller_sp = emu.sp();
    assert!(emu.stack_push64(RETURN));
    assert!(emu.set_rip_with_check(export, false));
    assert_eq!(emu.pc(), HANDLER);
    let pointers = emu.regs().rcx;
    assert_eq!(emu.maps.read_qword(pointers), Some(RECORD));
    let context = emu.maps.read_qword(pointers + 8).unwrap();
    assert_eq!(
        emu.maps
            .read_qword(RECORD + ExceptionRecord::ADDRESS64_OFFSET),
        Some(RETURN)
    );
    assert_eq!(
        emu.maps.read_qword(context + Context64::RIP_OFFSET),
        Some(RETURN)
    );
    assert_eq!(
        emu.maps.read_qword(context + Context64::RSP_OFFSET),
        Some(caller_sp)
    );
    emu.cfg.max_instructions = Some(20);
    assert_eq!(emu.run(Some(RETURN)).unwrap(), RETURN);
    assert_eq!((emu.pc(), emu.sp(), emu.eh_ctx()), (RETURN, caller_sp, 0));
    assert!(!emu.process_terminated);
}

#[test]
fn unhandled_software_exceptions_terminate_the_guest_at_both_exports() {
    use crate::api::windows::winapi64;
    use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;

    const IMAGE: u64 = 0x4100_0000;
    const RECORD: u64 = IMAGE + 0x6000;
    const CALLER: u64 = IMAGE + 0x8000;
    for (library, api) in [
        ("kernel32.dll", "RaiseException"),
        ("ntdll.dll", "RtlRaiseException"),
    ] {
        let mut emu = emu64();
        emu.init_stack64();
        build_minimal_pe64(&mut emu, IMAGE, 0x5000);
        emu.maps.write_dword(RECORD, STATUS_ACCESS_VIOLATION as u32);
        emu.regs_mut().rcx = if api == "RaiseException" {
            STATUS_ACCESS_VIOLATION
        } else {
            RECORD
        };
        emu.set_pc(CALLER);
        winapi64::gateway_by_import(&mut emu, library, api, 0);
        assert!(
            emu.process_terminated,
            "{api} resumed an unhandled exception"
        );
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.pc(), CALLER);
    }
}

#[test]
fn c_language_handler_calls_the_filter_and_only_unwinds_an_accepted_scope() {
    use crate::context::context64::Context64;
    use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;
    use crate::windows::constants::exceptions::{EXCEPTION_TARGET_UNWIND, EXCEPTION_UNWINDING};
    const IMAGE: u64 = 0x4100_0000;
    const AT: u64 = IMAGE + 0x2500;
    const CATCH: u32 = 0x2600;
    const LANGUAGE: u32 = 0x5000;
    const FILTER: u32 = 0x5100;
    const FINALLY: u32 = 0x5200;
    const COUNT: u64 = IMAGE + 0x7000;
    enum Case {
        Resume,
        Search,
        Catch,
        InvalidFilter,
        CatchSuppliedContext,
    }
    use Case::*;
    for case in [Resume, Search, Catch, InvalidFilter, CatchSuppliedContext] {
        let valid_filter = !matches!(case, InvalidFilter);
        let result: i32 = match case {
            Resume => -1,
            Search => 0,
            _ => 1,
        };
        let mut emu = emu64();
        emu.init_stack64();
        emu.running_script = true;
        emu.cfg.console_enabled = false;
        let ui = build_minimal_pe64(&mut emu, IMAGE, LANGUAGE);
        emu.maps
            .write_byte(ui, 1 | (UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER) << 3);
        let table = ui + 8;
        emu.maps.write_dword(table, 2);
        // The inner finally runs during target unwind, before the outer except block.
        for (index, handler, jump) in [
            (0, FINALLY, 0),
            (1, if valid_filter { FILTER } else { 0x10_0000 }, CATCH),
        ] {
            let at = table + 4 + index * 16;
            for (offset, value) in [(0, 0x2400), (4, CATCH), (8, handler), (12, jump)] {
                emu.maps.write_dword(at + offset, value);
            }
        }
        emu.create_stub_pe64("ntdll.dll", &["__C_specific_handler"]);
        let export = emu.maps.get_mem("ntdll.text").get_base();
        // The compiler's language-handler thunk tail jumps to the imported runtime.
        let mut thunk = vec![0x48, 0xb8];
        thunk.extend_from_slice(&export.to_le_bytes());
        thunk.extend_from_slice(&[0xff, 0xe0]);
        emu.maps.write_bytes(IMAGE + u64::from(LANGUAGE), &thunk);
        let mut filter = vec![0xb8];
        filter.extend_from_slice(&result.to_le_bytes());
        filter.push(0xc3);
        emu.maps.write_bytes(IMAGE + u64::from(FILTER), &filter);
        // mov rax,COUNT; inc dword ptr [rax]; ret
        let mut finally = vec![0x48, 0xb8];
        finally.extend_from_slice(&COUNT.to_le_bytes());
        finally.extend_from_slice(&[0xff, 0x00, 0xc3]);
        emu.maps.write_bytes(IMAGE + u64::from(FINALLY), &finally);
        emu.set_pc(AT);
        let sp = emu.sp();
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if pc == IMAGE + u64::from(FILTER) {
                let pointers = emu.regs().rcx;
                let rec = emu.maps.read_qword(pointers).unwrap();
                let context = emu.maps.read_qword(pointers + 8).unwrap();
                assert_eq!(
                    emu.maps.read_dword(rec),
                    Some(STATUS_ACCESS_VIOLATION as u32)
                );
                assert_eq!(
                    emu.maps.read_qword(context + Context64::RIP_OFFSET),
                    Some(AT)
                );
                assert_eq!(emu.regs().rdx, sp);
            } else if pc == IMAGE + u64::from(FINALLY) {
                assert_eq!((emu.regs().rcx, emu.regs().rdx), (1, sp));
                let rec = exception::handlers::active_record(emu).unwrap();
                let flags = emu
                    .maps
                    .read_dword(rec + ExceptionRecord::FLAGS_OFFSET)
                    .unwrap();
                assert_eq!(
                    flags & (EXCEPTION_UNWINDING | EXCEPTION_TARGET_UNWIND),
                    EXCEPTION_UNWINDING | EXCEPTION_TARGET_UNWIND
                );
            }
            true
        });
        if matches!(case, CatchSuppliedContext) {
            use crate::api::windows::winapi64;
            use crate::exception::unwind64::capture_context;
            let record = IMAGE + 0x8000;
            let context = IMAGE + 0x8100;
            capture_context(&mut emu, context, AT).unwrap();
            emu.maps.write_dword(record, STATUS_ACCESS_VIOLATION as u32);
            emu.set_pc(IMAGE + 0x9000);
            emu.set_sp(sp - 0x80);
            emu.regs_mut().rcx = record;
            emu.regs_mut().rdx = context;
            winapi64::gateway_by_import(&mut emu, "ntdll.dll", "RtlDispatchException", 0);
        } else {
            emu.exception(ExceptionType::BadAddressDereferencing);
        }
        assert_eq!(emu.pc(), IMAGE + u64::from(LANGUAGE));
        for _ in 0..8 {
            if emu.eh_ctx() == 0 || emu.api_stopped() || emu.process_terminated {
                break;
            }
            emu.step();
        }
        if !valid_filter {
            assert!(emu.api_stopped());
            assert_ne!(emu.pc(), IMAGE + u64::from(CATCH));
            assert_eq!(emu.maps.read_dword(COUNT), Some(0));
            continue;
        }
        assert!(!emu.api_stopped(), "{:?}", emu.api_policy.last_outcome);
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.process_terminated, result == 0);
        let expected_pc = if result > 0 {
            IMAGE + u64::from(CATCH)
        } else {
            AT
        };
        assert_eq!((emu.pc(), emu.sp()), (expected_pc, sp));
        assert_eq!(emu.maps.read_dword(COUNT), Some(u32::from(result > 0)));
    }
}
