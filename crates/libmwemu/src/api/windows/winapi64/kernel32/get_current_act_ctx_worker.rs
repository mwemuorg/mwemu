use crate::emu;

pub fn GetCurrentActCtxWorker(emu: &mut emu::Emu) {
    let act_ctx_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetCurrentActCtxWorker ptr:0x{:x}",
        act_ctx_ptr
    );
    if act_ctx_ptr != 0 {
        emu.maps.write_qword(act_ctx_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
