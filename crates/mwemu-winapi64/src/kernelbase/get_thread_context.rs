use crate::emu;

pub fn GetThreadContext(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;
    if ctx_ptr != 0 {
        // Write current register state to CONTEXT structure
        // CONTEXT64: ContextFlags at offset 0x30
        emu.maps.write_dword(ctx_ptr + 0x30, 0x10001F); // CONTEXT_ALL
        // Rax at 0x78, Rcx at 0x80, Rdx at 0x88, Rbx at 0x90
        emu.maps.write_qword(ctx_ptr + 0x78, emu.regs().rax);
        emu.maps.write_qword(ctx_ptr + 0x80, emu.regs().rcx);
        emu.maps.write_qword(ctx_ptr + 0x88, emu.regs().rdx);
        emu.maps.write_qword(ctx_ptr + 0x90, emu.regs().rbx);
        emu.maps.write_qword(ctx_ptr + 0x98, emu.regs().rsp);
        emu.maps.write_qword(ctx_ptr + 0xA0, emu.regs().rbp);
        emu.maps.write_qword(ctx_ptr + 0xA8, emu.regs().rsi);
        emu.maps.write_qword(ctx_ptr + 0xB0, emu.regs().rdi);
        emu.maps.write_qword(ctx_ptr + 0xF8, emu.regs().rip);
    }
    log_red!(
        emu,
        "kernelbase!GetThreadContext h:0x{:x} ctx:0x{:x}",
        handle,
        ctx_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
