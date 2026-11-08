use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitializeContext(emu: &mut emu::Emu) {
    let buffer = arg(emu, 0) as u64;
    let context_flags = arg(emu, 1) as u64;
    let pp_context = arg(emu, 2) as u64;
    let p_length = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!InitializeContext buf:0x{:x} flags:0x{:x} ppCtx:0x{:x} pLen:0x{:x}",
        buffer,
        context_flags,
        pp_context,
        p_length
    );

    if p_length != 0 {
        emu.maps.write_dword(p_length, 1232);
    }
    if pp_context != 0 && buffer != 0 {
        emu.maps.write_qword(pp_context, buffer);
        emu.maps.write_dword(buffer, context_flags as u32);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
