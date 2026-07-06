use crate::context::context64;
use crate::emu;

pub fn GetThreadContext(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;

    let ctx = context64::Context64::new(emu.regs());
    ctx.save(ctx_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!GetThreadContext");

    emu.regs_mut().rax = 1;
}
