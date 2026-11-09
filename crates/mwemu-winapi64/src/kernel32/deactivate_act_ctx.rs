use crate::emu;

pub fn DeactivateActCtx(emu: &mut emu::Emu) {
    let act_ctx = emu.regs().rcx;
    let cookie = emu.regs().rdx;

    log_red!(emu, "kernel32!DeactivateActCtx ctx:0x{:x}", act_ctx);

    if cookie != 0 {
        emu.maps.write_qword(cookie, 0x1234);
    }
    emu.regs_mut().rax = 1; // TRUE
}
