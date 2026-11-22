use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::constants::exceptions::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "RtlUnwindEx" => RtlUnwindEx(emu),
        "RtlUnwind" => RtlUnwind(emu),
        "RtlVirtualUnwind" => RtlVirtualUnwind(emu),
        "RtlDispatchException" => RtlDispatchException(emu),
        "RtlRaiseException" => RtlRaiseException(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

/// VOID RtlUnwindEx(
///     PVOID             TargetFrame,       // rcx
///     PVOID             TargetIp,          // rdx
///     PEXCEPTION_RECORD ExceptionRecord,   // r8
///     PVOID             ReturnValue,       // r9
///     PCONTEXT          OriginalContext,    // [rsp+0x28]
///     PUNWIND_HISTORY_TABLE HistoryTable   // [rsp+0x30]
/// );
fn RtlUnwindEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let target_frame = api_try!(emu, call.arg(emu, 0));
    let target_ip = api_try!(emu, call.arg(emu, 1));
    let _exception_record = api_try!(emu, call.arg(emu, 2));
    let return_value = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "ntdll!RtlUnwindEx target_frame:0x{:x} target_ip:0x{:x} ret_val:0x{:x}",
        target_frame,
        target_ip,
        return_value,
    );

    let current_rsp = emu.regs().rsp;
    let current_rip = emu.pc();

    // Walk frames from current to target, executing UWOP codes to unwind each.
    let frames_unwound = unwind_frames(emu, current_rip, current_rsp, target_frame);

    log::trace!(
        "RtlUnwindEx: unwound {} frames, jumping to 0x{:x}",
        frames_unwound,
        target_ip,
    );

    // Set the return value and transfer control.
    // Override both rip and gateway_return so the engine resumes at target_ip
    // instead of the original call's return address.
    emu.regs_mut().rax = return_value;
    if target_frame != 0 {
        emu.regs_mut().rsp = target_frame;
    }
    emu.regs_mut().rip = target_ip;
    emu.gateway_return = target_ip;
}

/// VOID RtlUnwind(
///     PVOID             TargetFrame,       // rcx
///     PVOID             TargetIp,          // rdx
///     PEXCEPTION_RECORD ExceptionRecord,   // r8
///     PVOID             ReturnValue        // r9
/// );
fn RtlUnwind(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let target_frame = api_try!(emu, call.arg(emu, 0));
    let target_ip = api_try!(emu, call.arg(emu, 1));
    let _exception_record = api_try!(emu, call.arg(emu, 2));
    let return_value = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "ntdll!RtlUnwind target_frame:0x{:x} target_ip:0x{:x} ret_val:0x{:x}",
        target_frame,
        target_ip,
        return_value,
    );

    let current_rsp = emu.regs().rsp;
    let current_rip = emu.pc();
    let frames_unwound = unwind_frames(emu, current_rip, current_rsp, target_frame);

    log::trace!(
        "RtlUnwind: unwound {} frames, jumping to 0x{:x}",
        frames_unwound,
        target_ip,
    );

    emu.regs_mut().rax = return_value;
    if target_frame != 0 {
        emu.regs_mut().rsp = target_frame;
    }
    if target_ip != 0 {
        emu.regs_mut().rip = target_ip;
        emu.gateway_return = target_ip;
    }
}

/// PRUNTIME_FUNCTION RtlVirtualUnwind(
///     ULONG             HandlerType,       // rcx
///     ULONG64           ImageBase,          // rdx
///     ULONG64           ControlPc,          // r8
///     PRUNTIME_FUNCTION FunctionEntry,      // r9
///     PCONTEXT          ContextRecord,      // [rsp+0x28]
///     PVOID            *HandlerData,        // [rsp+0x30]
///     PULONG64          EstablisherFrame,   // [rsp+0x38]
///     PKNONVOLATILE_CONTEXT_POINTERS ContextPointers // [rsp+0x40]
/// );
fn RtlVirtualUnwind(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _handler_type = api_try!(emu, call.arg_u32(emu, 0));
    let image_base = api_try!(emu, call.arg(emu, 1));
    let _control_pc = api_try!(emu, call.arg(emu, 2));
    let function_entry = api_try!(emu, call.arg(emu, 3));
    let context_record = api_try!(emu, call.arg(emu, 4));
    let _handler_data = api_try!(emu, call.arg(emu, 5));
    let establisher_frame = api_try!(emu, call.arg(emu, 6));

    log_red!(
        emu,
        "ntdll!RtlVirtualUnwind base:0x{:x} pc:0x{:x} entry:0x{:x} ctx:0x{:x}",
        image_base,
        _control_pc,
        function_entry,
        context_record,
    );

    let unwind_rva = match emu.maps.read_dword(function_entry + 8) {
        Some(v) => v as u64,
        None => {
            call.set_return(emu, 0);
            return;
        }
    };

    let ui_addr = image_base + unwind_rva;

    // Execute unwind codes on the CONTEXT record in guest memory, not live regs.
    if context_record != 0 {
        if execute_unwind_codes_on_context(emu, ui_addr, context_record).is_none() {
            log::warn!(
                "RtlVirtualUnwind: failed to execute unwind codes at 0x{:x}",
                ui_addr,
            );
        }

        // Write *EstablisherFrame = ContextRecord.Rsp
        if establisher_frame != 0 {
            let new_rsp = emu.maps.read_qword(context_record + CTX_RSP).unwrap_or(0);
            emu.maps.write_qword(establisher_frame, new_rsp);
        }
    }

    // Return NULL (no language-specific handler found by this stub).
    call.set_return(emu, 0);
}

/// BOOLEAN RtlDispatchException(
///     PEXCEPTION_RECORD ExceptionRecord,  // rcx
///     PCONTEXT          ContextRecord      // rdx
/// );
/// Walks frames via .pdata and calls each EHANDLER. Returns TRUE (1) if a
/// handler took the exception, FALSE (0) otherwise.
fn RtlDispatchException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let record_addr = api_try!(emu, call.arg(emu, 0));
    let context_addr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "ntdll!RtlDispatchException record=0x{:x} context=0x{:x}",
        record_addr,
        context_addr,
    );

    let handled = emu.dispatch_x64_seh_frames(record_addr, context_addr);

    if handled {
        let saved = *emu.regs();
        let mut ctx = crate::context::context64::Context64::new(&saved);
        ctx.load(context_addr, &mut emu.maps);
        ctx.sync(emu.regs_mut());
    }

    call.set_return(emu, handled as u64);
}

/// VOID RtlRaiseException(
///     PEXCEPTION_RECORD ExceptionRecord   // rcx
/// );
/// Dispatches the exception through VEH then SEH. If no handler takes it,
/// the process is terminated.
fn RtlRaiseException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let record_addr = api_try!(emu, call.arg(emu, 0));

    let code = emu.maps.read_dword(record_addr).unwrap_or(0);
    let flags = emu.maps.read_dword(record_addr + 4).unwrap_or(0);
    log_red!(
        emu,
        "ntdll!RtlRaiseException record=0x{:x} code=0x{:x} flags=0x{:x}",
        record_addr,
        code,
        flags,
    );

    // Allocate a CONTEXT, populate it, and try VEH then SEH dispatch.
    const BLOCK_SIZE: u64 = 0x800;
    const CTX_OFF: u64 = 0x10;
    let Some(block) = emu.maps.alloc(BLOCK_SIZE) else {
        log::error!("RtlRaiseException: out of memory");
        emu.stop();
        return;
    };
    let map_name = format!("rtl_raise_{:x}", block);
    if emu
        .maps
        .create_map(&map_name, block, BLOCK_SIZE, Permission::READ_WRITE)
        .is_err()
    {
        log::error!("RtlRaiseException: cannot map context block");
        emu.stop();
        return;
    }

    let context_addr = block + CTX_OFF;
    let ctx = crate::context::context64::Context64::new(emu.regs());
    ctx.save(context_addr, &mut emu.maps);
    let eflags = emu.flags().dump();
    emu.maps.write_dword(
        context_addr + crate::context::context64::Context64::FLAGS_OFFSET,
        crate::windows::constants::exceptions::CONTEXT_CONTROL
            | crate::windows::constants::exceptions::CONTEXT_INTEGER,
    );
    emu.maps.write_dword(
        context_addr + crate::context::context64::Context64::EFLAGS_OFFSET,
        eflags,
    );

    // Try VEH first.
    let veh_handled = {
        let handlers = emu.veh_list().to_vec();
        let mut handled = false;
        // Build EXCEPTION_POINTERS: [record_addr, context_addr]
        emu.maps.write_qword(block, record_addr);
        emu.maps.write_qword(block + 8, context_addr);
        for handler in handlers {
            match emu.call64(handler, &[block]) {
                Ok(disp) if disp as u32 == EXCEPTION_CONTINUE_EXECUTION32 => {
                    handled = true;
                    break;
                }
                Err(e) => {
                    log::warn!("VEH 0x{:x} failed: {}", handler, e.message);
                    break;
                }
                _ => {}
            }
        }
        handled
    };

    if veh_handled {
        let saved = *emu.regs();
        let mut ctx = crate::context::context64::Context64::new(&saved);
        ctx.load(context_addr, &mut emu.maps);
        ctx.sync(emu.regs_mut());
        emu.maps.free(&map_name);
        return;
    }

    // Try SEH dispatch.
    let seh_handled = emu.dispatch_x64_seh_frames(record_addr, context_addr);

    if seh_handled {
        let saved = *emu.regs();
        let mut ctx = crate::context::context64::Context64::new(&saved);
        ctx.load(context_addr, &mut emu.maps);
        ctx.sync(emu.regs_mut());
    } else {
        log::error!(
            "RtlRaiseException: unhandled exception 0x{:x} at 0x{:x}",
            code,
            emu.pc(),
        );
    }

    emu.maps.free(&map_name);
}

// ---- CONTEXT64 field offsets (from winnt.h) --------------------------------

const CTX_RAX: u64 = 120;
const CTX_RCX: u64 = 128;
const CTX_RDX: u64 = 136;
const CTX_RBX: u64 = 144;
const CTX_RSP: u64 = 152;
const CTX_RBP: u64 = 160;
const CTX_RSI: u64 = 168;
const CTX_RDI: u64 = 176;
const CTX_R8: u64 = 184;
const CTX_R9: u64 = 192;
const CTX_R10: u64 = 200;
const CTX_R11: u64 = 208;
const CTX_R12: u64 = 216;
const CTX_R13: u64 = 224;
const CTX_R14: u64 = 232;
const CTX_R15: u64 = 240;
const CTX_RIP: u64 = 248;

fn ctx_reg_offset(index: u8) -> Option<u64> {
    match index {
        0 => Some(CTX_RAX),
        1 => Some(CTX_RCX),
        2 => Some(CTX_RDX),
        3 => Some(CTX_RBX),
        4 => Some(CTX_RSP),
        5 => Some(CTX_RBP),
        6 => Some(CTX_RSI),
        7 => Some(CTX_RDI),
        8 => Some(CTX_R8),
        9 => Some(CTX_R9),
        10 => Some(CTX_R10),
        11 => Some(CTX_R11),
        12 => Some(CTX_R12),
        13 => Some(CTX_R13),
        14 => Some(CTX_R14),
        15 => Some(CTX_R15),
        _ => None,
    }
}

// ---- unwind engine --------------------------------------------------------

const MAX_FRAMES: usize = 256;

/// Walk stack frames from `rip`/`rsp` toward `target_frame`, executing UWOP
/// codes for each frame to restore nonvolatile registers and adjust RSP.
/// Returns the number of frames successfully unwound.
fn unwind_frames(emu: &mut emu::Emu, mut rip: u64, mut rsp: u64, target_frame: u64) -> usize {
    for depth in 0..MAX_FRAMES {
        if target_frame != 0 && rsp >= target_frame {
            return depth;
        }

        let Some((pe_base, entry_addr)) = emu.runtime_function_entry(rip) else {
            // No .pdata entry — leaf function. Return address is at [rsp].
            match emu.maps.read_qword(rsp) {
                Some(ret_addr) => {
                    rsp += 8;
                    rip = ret_addr;
                    emu.regs_mut().rsp = rsp;
                    continue;
                }
                None => return depth,
            }
        };

        let unwind_rva = match emu.maps.read_dword(entry_addr + 8) {
            Some(v) => v as u64,
            None => return depth,
        };

        let ui_addr = pe_base + unwind_rva;
        if execute_unwind_codes(emu, ui_addr).is_none() {
            return depth;
        }

        // After unwind codes, RSP points at the return address.
        rsp = emu.regs().rsp;
        match emu.maps.read_qword(rsp) {
            Some(ret_addr) => {
                rsp += 8;
                rip = ret_addr;
                emu.regs_mut().rsp = rsp;
            }
            None => return depth,
        }
    }
    MAX_FRAMES
}

// UWOP codes as defined in the PE x64 exception handling spec.
const UWOP_PUSH_NONVOL: u8 = 0;
const UWOP_ALLOC_LARGE: u8 = 1;
const UWOP_ALLOC_SMALL: u8 = 2;
const UWOP_SET_FPREG: u8 = 3;
const UWOP_SAVE_NONVOL: u8 = 4;
const UWOP_SAVE_NONVOL_FAR: u8 = 5;
const UWOP_SAVE_XMM128: u8 = 8;
const UWOP_SAVE_XMM128_FAR: u8 = 9;
const UWOP_PUSH_MACHFRAME: u8 = 10;

/// Execute the unwind codes from an UNWIND_INFO structure, restoring registers
/// and adjusting RSP. Follows CHAININFO links. Returns Some(()) on success.
fn execute_unwind_codes(emu: &mut emu::Emu, mut ui_addr: u64) -> Option<()> {
    const MAX_CHAIN: usize = 16;

    for _ in 0..MAX_CHAIN {
        let version_flags = emu.maps.read_byte(ui_addr)?;
        let _version = version_flags & 0x07;
        let flags = (version_flags >> 3) & 0x1F;
        let _size_of_prolog = emu.maps.read_byte(ui_addr + 1)?;
        let count_of_codes = emu.maps.read_byte(ui_addr + 2)? as u64;
        let frame_reg_off = emu.maps.read_byte(ui_addr + 3)?;
        let frame_register = frame_reg_off & 0x0F;
        let frame_offset = ((frame_reg_off >> 4) & 0x0F) as u64 * 16;

        let codes_base = ui_addr + 4;
        let mut i: u64 = 0;

        while i < count_of_codes {
            let code_word = emu.maps.read_word(codes_base + i * 2)?;
            let op = (code_word >> 8) & 0x0F;
            let info = (code_word >> 12) & 0x0F;
            let op = op as u8;
            let info = info as u8;

            match op {
                UWOP_PUSH_NONVOL => {
                    // Reverse a push: pop from [rsp] into the register.
                    let val = emu.maps.read_qword(emu.regs().rsp)?;
                    set_unwind_reg(emu, info, val);
                    emu.regs_mut().rsp += 8;
                    i += 1;
                }
                UWOP_ALLOC_LARGE => {
                    if info == 0 {
                        // Next slot is size / 8.
                        let next = emu.maps.read_word(codes_base + (i + 1) * 2)? as u64;
                        emu.regs_mut().rsp += next * 8;
                        i += 2;
                    } else {
                        // Next two slots are a 32-bit size.
                        let lo = emu.maps.read_word(codes_base + (i + 1) * 2)? as u32;
                        let hi = emu.maps.read_word(codes_base + (i + 2) * 2)? as u32;
                        let size = ((hi << 16) | lo) as u64;
                        emu.regs_mut().rsp += size;
                        i += 3;
                    }
                }
                UWOP_ALLOC_SMALL => {
                    emu.regs_mut().rsp += (info as u64) * 8 + 8;
                    i += 1;
                }
                UWOP_SET_FPREG => {
                    if frame_register != 0 {
                        let fp_val = get_unwind_reg(emu, frame_register);
                        emu.regs_mut().rsp = fp_val.wrapping_sub(frame_offset);
                    }
                    i += 1;
                }
                UWOP_SAVE_NONVOL => {
                    let offset = emu.maps.read_word(codes_base + (i + 1) * 2)? as u64 * 8;
                    let val = emu.maps.read_qword(emu.regs().rsp + offset)?;
                    set_unwind_reg(emu, info, val);
                    i += 2;
                }
                UWOP_SAVE_NONVOL_FAR => {
                    let lo = emu.maps.read_word(codes_base + (i + 1) * 2)? as u32;
                    let hi = emu.maps.read_word(codes_base + (i + 2) * 2)? as u32;
                    let offset = ((hi << 16) | lo) as u64;
                    let val = emu.maps.read_qword(emu.regs().rsp + offset)?;
                    set_unwind_reg(emu, info, val);
                    i += 3;
                }
                UWOP_SAVE_XMM128 => {
                    // Skip restoring XMM for now — just advance past the slot.
                    i += 2;
                }
                UWOP_SAVE_XMM128_FAR => {
                    i += 3;
                }
                UWOP_PUSH_MACHFRAME => {
                    // Machine frame pushed by hardware interrupt/exception.
                    let base = emu.regs().rsp;
                    if info == 1 {
                        // Error code present, skip it.
                        emu.regs_mut().rsp += 8;
                    }
                    let new_rip = emu.maps.read_qword(emu.regs().rsp)?;
                    let new_rsp = emu.maps.read_qword(emu.regs().rsp + 24)?;
                    emu.regs_mut().rip = new_rip;
                    emu.regs_mut().rsp = new_rsp;
                    log::trace!(
                        "UWOP_PUSH_MACHFRAME: restored rip=0x{:x} rsp=0x{:x} from 0x{:x}",
                        new_rip,
                        new_rsp,
                        base,
                    );
                    i += 1;
                }
                _ => {
                    log::warn!("unknown UWOP code {} at 0x{:x}", op, codes_base + i * 2);
                    return None;
                }
            }
        }

        // Check for CHAININFO — the chained RUNTIME_FUNCTION follows the codes.
        if flags & UNW_FLAG_CHAININFO != 0 {
            let codes_end = codes_base + count_of_codes * 2;
            let aligned = (codes_end + 3) & !3;
            let chained_unwind_rva = emu.maps.read_dword(aligned + 8)? as u64;
            let (pe_base, _) = emu.maps.find_pe_image_info(ui_addr)?;
            ui_addr = pe_base + chained_unwind_rva;
            continue;
        }

        return Some(());
    }

    log::warn!("UNWIND_INFO chain depth exceeded at 0x{:x}", ui_addr);
    None
}

/// Like execute_unwind_codes but operates on a CONTEXT record in guest memory
/// instead of the live emulator registers. Used by RtlVirtualUnwind.
fn execute_unwind_codes_on_context(emu: &mut emu::Emu, mut ui_addr: u64, ctx: u64) -> Option<()> {
    const MAX_CHAIN: usize = 16;

    for _ in 0..MAX_CHAIN {
        let version_flags = emu.maps.read_byte(ui_addr)?;
        let flags = (version_flags >> 3) & 0x1F;
        let count_of_codes = emu.maps.read_byte(ui_addr + 2)? as u64;
        let frame_reg_off = emu.maps.read_byte(ui_addr + 3)?;
        let frame_register = frame_reg_off & 0x0F;
        let frame_offset = ((frame_reg_off >> 4) & 0x0F) as u64 * 16;

        let codes_base = ui_addr + 4;
        let mut i: u64 = 0;

        while i < count_of_codes {
            let code_word = emu.maps.read_word(codes_base + i * 2)?;
            let op = ((code_word >> 8) & 0x0F) as u8;
            let info = ((code_word >> 12) & 0x0F) as u8;

            match op {
                UWOP_PUSH_NONVOL => {
                    let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    let val = emu.maps.read_qword(rsp_val)?;
                    if let Some(off) = ctx_reg_offset(info) {
                        emu.maps.write_qword(ctx + off, val);
                    }
                    emu.maps.write_qword(ctx + CTX_RSP, rsp_val + 8);
                    i += 1;
                }
                UWOP_ALLOC_LARGE => {
                    let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    if info == 0 {
                        let next = emu.maps.read_word(codes_base + (i + 1) * 2)? as u64;
                        emu.maps.write_qword(ctx + CTX_RSP, rsp_val + next * 8);
                        i += 2;
                    } else {
                        let lo = emu.maps.read_word(codes_base + (i + 1) * 2)? as u32;
                        let hi = emu.maps.read_word(codes_base + (i + 2) * 2)? as u32;
                        let size = ((hi << 16) | lo) as u64;
                        emu.maps.write_qword(ctx + CTX_RSP, rsp_val + size);
                        i += 3;
                    }
                }
                UWOP_ALLOC_SMALL => {
                    let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    emu.maps
                        .write_qword(ctx + CTX_RSP, rsp_val + (info as u64) * 8 + 8);
                    i += 1;
                }
                UWOP_SET_FPREG => {
                    if frame_register != 0
                        && let Some(off) = ctx_reg_offset(frame_register)
                    {
                        let fp_val = emu.maps.read_qword(ctx + off)?;
                        emu.maps
                            .write_qword(ctx + CTX_RSP, fp_val.wrapping_sub(frame_offset));
                    }
                    i += 1;
                }
                UWOP_SAVE_NONVOL => {
                    let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    let offset = emu.maps.read_word(codes_base + (i + 1) * 2)? as u64 * 8;
                    let val = emu.maps.read_qword(rsp_val + offset)?;
                    if let Some(off) = ctx_reg_offset(info) {
                        emu.maps.write_qword(ctx + off, val);
                    }
                    i += 2;
                }
                UWOP_SAVE_NONVOL_FAR => {
                    let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    let lo = emu.maps.read_word(codes_base + (i + 1) * 2)? as u32;
                    let hi = emu.maps.read_word(codes_base + (i + 2) * 2)? as u32;
                    let offset = ((hi << 16) | lo) as u64;
                    let val = emu.maps.read_qword(rsp_val + offset)?;
                    if let Some(off) = ctx_reg_offset(info) {
                        emu.maps.write_qword(ctx + off, val);
                    }
                    i += 3;
                }
                UWOP_SAVE_XMM128 => {
                    i += 2;
                }
                UWOP_SAVE_XMM128_FAR => {
                    i += 3;
                }
                UWOP_PUSH_MACHFRAME => {
                    let mut rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
                    if info == 1 {
                        rsp_val += 8;
                    }
                    let new_rip = emu.maps.read_qword(rsp_val)?;
                    let new_rsp = emu.maps.read_qword(rsp_val + 24)?;
                    emu.maps.write_qword(ctx + CTX_RIP, new_rip);
                    emu.maps.write_qword(ctx + CTX_RSP, new_rsp);
                    i += 1;
                }
                _ => {
                    log::warn!("unknown UWOP code {} at 0x{:x}", op, codes_base + i * 2);
                    return None;
                }
            }
        }

        if flags & UNW_FLAG_CHAININFO != 0 {
            let codes_end = codes_base + count_of_codes * 2;
            let aligned = (codes_end + 3) & !3;
            let chained_unwind_rva = emu.maps.read_dword(aligned + 8)? as u64;
            let (pe_base, _) = emu.maps.find_pe_image_info(ui_addr)?;
            ui_addr = pe_base + chained_unwind_rva;
            continue;
        }

        // After unwind codes, update RIP from the return address at [new_rsp].
        let rsp_val = emu.maps.read_qword(ctx + CTX_RSP)?;
        if let Some(ret_addr) = emu.maps.read_qword(rsp_val) {
            emu.maps.write_qword(ctx + CTX_RIP, ret_addr);
            emu.maps.write_qword(ctx + CTX_RSP, rsp_val + 8);
        }

        return Some(());
    }

    log::warn!("UNWIND_INFO chain depth exceeded at 0x{:x}", ui_addr);
    None
}

/// Map UWOP register index to the actual x64 nonvolatile register.
/// Indices: 0=RAX, 1=RCX, 2=RDX, 3=RBX, 4=RSP, 5=RBP, 6=RSI, 7=RDI,
///          8=R8, 9=R9, 10=R10, 11=R11, 12=R12, 13=R13, 14=R14, 15=R15.
fn set_unwind_reg(emu: &mut emu::Emu, index: u8, value: u64) {
    match index {
        0 => emu.regs_mut().rax = value,
        1 => emu.regs_mut().rcx = value,
        2 => emu.regs_mut().rdx = value,
        3 => emu.regs_mut().rbx = value,
        4 => emu.regs_mut().rsp = value,
        5 => emu.regs_mut().rbp = value,
        6 => emu.regs_mut().rsi = value,
        7 => emu.regs_mut().rdi = value,
        8 => emu.regs_mut().r8 = value,
        9 => emu.regs_mut().r9 = value,
        10 => emu.regs_mut().r10 = value,
        11 => emu.regs_mut().r11 = value,
        12 => emu.regs_mut().r12 = value,
        13 => emu.regs_mut().r13 = value,
        14 => emu.regs_mut().r14 = value,
        15 => emu.regs_mut().r15 = value,
        _ => {}
    }
}

fn get_unwind_reg(emu: &emu::Emu, index: u8) -> u64 {
    match index {
        0 => emu.regs().rax,
        1 => emu.regs().rcx,
        2 => emu.regs().rdx,
        3 => emu.regs().rbx,
        4 => emu.regs().rsp,
        5 => emu.regs().rbp,
        6 => emu.regs().rsi,
        7 => emu.regs().rdi,
        8 => emu.regs().r8,
        9 => emu.regs().r9,
        10 => emu.regs().r10,
        11 => emu.regs().r11,
        12 => emu.regs().r12,
        13 => emu.regs().r13,
        14 => emu.regs().r14,
        15 => emu.regs().r15,
        _ => 0,
    }
}
