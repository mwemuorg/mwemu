use std::cell::RefCell;
use std::rc::Rc;

use crate::context::context64::Context64;
use crate::emu64;
use crate::maps::mem64::Permission;
use crate::windows::constants::exceptions::{
    EXCEPTION_UNWINDING, ExceptionDisposition, UNW_FLAG_EHANDLER, UNW_FLAG_UHANDLER,
};

#[test]
fn guest_exception_handler_unwinds_to_an_outer_frame_and_returns_to_its_caller() {
    const IMAGE: u64 = 0x4100_0000;
    const OUTER: u32 = 0x300;
    const TARGET: u32 = 0x320;
    const INNER: u32 = 0x380;
    const INNER_HANDLER: u32 = 0x500;
    const OUTER_HANDLER: u32 = 0x600;
    const SENTINEL: u32 = 0x700;
    const EXCEPTION_VALUE: u64 = 0x0000_1234_abcd_5678;

    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.init_stack64();
    let image = |rva: u32| IMAGE + u64::from(rva);
    emu.maps
        .create_map(
            "guest_unwind.pe",
            IMAGE,
            0x1000,
            Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    assert_ne!(emu.create_stub_pe64("ntdll.dll", &["RtlUnwindEx"]), 0);
    let unwind_export = emu.maps.get_mem("ntdll.text").get_base();
    let pe = IMAGE + 0x80;
    emu.maps.write_dword(IMAGE + 0x3c, 0x80);
    emu.maps.write_dword(pe + 0x50, 0x1000);
    emu.maps.write_dword(pe + 0x84, 16);
    emu.maps.write_dword(pe + 0xa0, 0x200);
    emu.maps.write_dword(pe + 0xa4, 3 * 12);

    // Each UWOP_ALLOC_SMALL describes the SUB RSP at the start of its function.
    let frames = [
        (
            OUTER,
            TARGET + 5,
            0x900,
            UNW_FLAG_EHANDLER,
            0x42,
            OUTER_HANDLER,
        ),
        (
            INNER,
            INNER + 8,
            0x910,
            UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER,
            0x42,
            INNER_HANDLER,
        ),
        (OUTER_HANDLER, OUTER_HANDLER + 0x60, 0x920, 0, 0x62, 0),
    ];
    for (index, (begin, end, unwind, flags, alloc_small, handler)) in frames.into_iter().enumerate()
    {
        let entry = IMAGE + 0x200 + index as u64 * 12;
        emu.maps.write_dword(entry, begin);
        emu.maps.write_dword(entry + 4, end);
        emu.maps.write_dword(entry + 8, unwind);
        emu.maps.write_bytes(
            image(unwind),
            &[1 | (flags << 3), 4, 1, 0, 4, alloc_small, 0, 0],
        );
        if handler != 0 {
            emu.maps.write_dword(image(unwind) + 8, handler);
        }
    }

    // Outer: SUB RSP,28h; CALL inner. Its exception continuation uses the same epilogue.
    let mut outer = vec![0x48, 0x83, 0xec, 0x28, 0xe8];
    outer.extend_from_slice(&(INNER - (OUTER + 9)).to_le_bytes());
    outer.push(0x90);
    emu.maps.write_bytes(image(OUTER), &outer);
    emu.maps
        .write_bytes(image(TARGET), &[0x48, 0x83, 0xc4, 0x28, 0xc3]);
    // Inner: SUB RSP,28h; UD2. The same language handler participates in both passes.
    emu.maps.write_bytes(
        image(INNER),
        &[0x48, 0x83, 0xec, 0x28, 0x0f, 0x0b, 0x90, 0x90],
    );
    let mut continue_search = vec![0xb8];
    continue_search.extend_from_slice(&(ExceptionDisposition::ContinueSearch as u32).to_le_bytes());
    continue_search.push(0xc3);
    emu.maps.write_bytes(image(INNER_HANDLER), &continue_search);

    // The outer language handler supplies the dispatcher-owned scratch CONTEXT and history table.
    let mut handler = vec![
        0x48, 0x83, 0xec, 0x38, // sub rsp,38h
        0x49, 0x8b, 0x41, 0x28, // mov rax,[r9+28h]  ; ContextRecord
        0x48, 0x89, 0x44, 0x24, 0x20, // mov [rsp+20h],rax
        0x49, 0x8b, 0x41, 0x40, // mov rax,[r9+40h]  ; HistoryTable
        0x48, 0x89, 0x44, 0x24, 0x28, // mov [rsp+28h],rax
        0x49, 0x89, 0xc8, // mov r8,rcx        ; exception record
        0x48, 0x89, 0xd1, // mov rcx,rdx       ; establisher frame
        0xba, // mov edx,target
    ];
    handler.extend_from_slice(&(image(TARGET) as u32).to_le_bytes());
    handler.extend_from_slice(&[0x49, 0xb9]); // mov r9,return value
    handler.extend_from_slice(&EXCEPTION_VALUE.to_le_bytes());
    handler.extend_from_slice(&[0x48, 0xb8]); // mov rax,ntdll!RtlUnwindEx
    handler.extend_from_slice(&unwind_export.to_le_bytes());
    handler.extend_from_slice(&[0xff, 0xd0]); // call rax
    let unexpected_return = image(OUTER_HANDLER) + handler.len() as u64;
    handler.extend_from_slice(&[0x48, 0x83, 0xc4, 0x38]);
    handler.extend_from_slice(&continue_search);
    emu.maps.write_bytes(image(OUTER_HANDLER), &handler);

    let visits = Rc::new(RefCell::new(Vec::new()));
    let observed = Rc::clone(&visits);
    let entry_sp = (emu.sp() & !15) - 8;
    emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
        assert_ne!(
            pc, unexpected_return,
            "targeted RtlUnwindEx returned to the handler"
        );
        if pc == image(INNER_HANDLER) || pc == image(OUTER_HANDLER) {
            let flags = emu
                .maps
                .read_dword(emu.regs().rcx + 4)
                .expect("exception flags");
            let dispatch = emu.regs().r9;
            let handler_context = emu.regs().r8;
            let cursor = emu
                .maps
                .read_qword(dispatch + 0x28)
                .expect("dispatcher ContextRecord");
            let fault_ip = image(INNER) + 4;
            let fault_sp = entry_sp - 0x58;
            let read_frame = |ctx| {
                (
                    emu.maps.read_qword(ctx + Context64::RIP_OFFSET),
                    emu.maps.read_qword(ctx + Context64::RSP_OFFSET),
                )
            };
            assert_ne!(cursor, 0);
            if flags & EXCEPTION_UNWINDING == 0 {
                assert_ne!(
                    cursor, handler_context,
                    "unwind scratch aliases the fault context"
                );
                let after_inner = image(OUTER) + 9;
                let (control, caller_ip, caller_sp) = if pc == image(INNER_HANDLER) {
                    (fault_ip, after_inner, entry_sp - 0x28)
                } else {
                    (after_inner, image(SENTINEL), entry_sp + 8)
                };
                assert_eq!(emu.maps.read_qword(dispatch), Some(control));
                assert_eq!(
                    read_frame(handler_context),
                    (Some(fault_ip), Some(fault_sp))
                );
                assert_eq!(read_frame(cursor), (Some(caller_ip), Some(caller_sp)));
            } else {
                assert_eq!(pc, image(INNER_HANDLER));
                assert_eq!(emu.regs().rdx, fault_sp);
                assert_eq!(emu.maps.read_qword(dispatch), Some(fault_ip));
                assert_eq!(emu.maps.read_qword(dispatch + 0x20), Some(image(TARGET)));
                assert_eq!(handler_context, cursor);
                assert_eq!(read_frame(cursor), (Some(fault_ip), Some(fault_sp)));
            }
            observed
                .borrow_mut()
                .push((pc, flags, emu.regs().rdx, emu.eh_ctx()));
        }
        true
    });
    emu.maps.write_qword(entry_sp, image(SENTINEL));
    emu.set_sp(entry_sp);
    emu.set_pc(image(OUTER));
    emu.cfg.max_instructions = Some(200);

    assert_eq!(
        emu.run(Some(image(TARGET))).expect("transfer to catch"),
        image(TARGET)
    );
    assert_eq!(
        (emu.sp(), emu.regs().rax),
        (entry_sp - 0x28, EXCEPTION_VALUE)
    );
    assert_eq!(emu.eh_ctx(), 0);
    {
        let visits = visits.borrow();
        let search: Vec<_> = visits
            .iter()
            .filter(|v| v.1 & EXCEPTION_UNWINDING == 0)
            .map(|v| (v.0, v.2))
            .collect();
        assert_eq!(
            search,
            [
                (image(INNER_HANDLER), entry_sp - 0x58),
                (image(OUTER_HANDLER), entry_sp - 0x28)
            ]
        );
        assert!(
            visits
                .iter()
                .any(|v| v.0 == image(INNER_HANDLER) && v.1 & EXCEPTION_UNWINDING != 0)
        );
        for &(_, _, _, context) in visits.iter().filter(|v| v.1 & EXCEPTION_UNWINDING == 0) {
            assert_ne!(context, 0);
            assert!(
                emu.maps.get_mem_by_addr(context).is_none(),
                "abandoned exception frame is still mapped"
            );
        }
    }
    assert_eq!(
        emu.run(Some(image(SENTINEL))).expect("return to caller"),
        image(SENTINEL)
    );
    assert_eq!(emu.sp(), entry_sp + 8);
    assert_eq!(emu.regs().rax, EXCEPTION_VALUE);
}
