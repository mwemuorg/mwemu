use crate::emu;

pub fn InitOnceComplete(emu: &mut emu::Emu) {
    let lp_init_once = emu.regs().rcx;
    let dw_flags = emu.regs().rdx as u32;
    let lp_context = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!InitOnceComplete once:0x{:x} flags:0x{:x} ctx:0x{:x}",
        lp_init_once,
        dw_flags,
        lp_context
    );

    // Mark the INIT_ONCE as completed (value 2 = initialized)
    if lp_init_once != 0 {
        emu.maps.write_qword(lp_init_once, 2);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
