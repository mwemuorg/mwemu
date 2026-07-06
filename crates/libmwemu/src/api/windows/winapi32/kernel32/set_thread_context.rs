use crate::context::context32;
use crate::debug::console;
use crate::emu;

pub fn SetThreadContext(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SetThreadContext cannot read the handle");
    let ctx_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!SetThreadContext cannot read the ctx_ptr");

    log_red!(emu, "kernel32!SetThreadContext");

    let con = console::Console::new();
    con.print("apply the context (y/n)?");
    let opt = con.cmd();

    if opt == "y" || opt == "yes" {
        let mut ctx = context32::Context32::new(emu.regs());
        ctx.load(ctx_ptr, &mut emu.maps);
        ctx.sync(emu.regs_mut());
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
