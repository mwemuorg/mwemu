use crate::context::context32;
use crate::emu;

pub fn GetThreadContext(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetThreadContext cannot read the handle");
    let ctx_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetThreadContext cannot read the ctx");

    let ctx = context32::Context32::new(emu.regs());
    ctx.save(ctx_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!GetThreadContext");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
