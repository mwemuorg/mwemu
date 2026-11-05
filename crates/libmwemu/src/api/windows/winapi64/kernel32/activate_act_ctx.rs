use crate::emu;

pub fn ActivateActCtx(emu: &mut emu::Emu) {
    let h_act_ctx = emu.regs().rcx;
    let lp_cookie = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!ActivateActCtx ctx:0x{:x} cookie:0x{:x}",
        h_act_ctx,
        lp_cookie
    );
    if lp_cookie != 0 {
        emu.maps.write_qword(lp_cookie, 0x1001);
    }
    emu.regs_mut().rax = 1;
}
