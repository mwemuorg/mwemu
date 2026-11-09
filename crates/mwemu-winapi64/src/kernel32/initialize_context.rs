use crate::emu;

pub fn InitializeContext(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let context_flags = emu.regs().rdx;
    let pp_context = emu.regs().r8;
    let p_length = emu.regs().r9;

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
}
