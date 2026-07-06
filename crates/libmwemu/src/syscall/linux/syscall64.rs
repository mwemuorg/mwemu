use crate::emu;

mod fs;
mod memory;
mod misc;
mod net;
mod proc;
mod signal;
/*
 * /usr/include/asm/unistd_64.h
 *
 *  params: RDI, RSI, RDX, R10, R8, R9
 *
 *
 */

fn trace_syscall64(emu: &mut emu::Emu, name: &str) {
    log::trace!(
        "{}** {} syscall {} {}",
        emu.colors.light_red,
        emu.pos,
        name,
        emu.colors.nc
    );
}

pub fn gateway(emu: &mut emu::Emu) {
    let nr = emu.regs().rax;
    if !emu.call_syscall_hook(nr) {
        return;
    }
    if fs::dispatch(emu)
        || proc::dispatch(emu)
        || net::dispatch(emu)
        || memory::dispatch(emu)
        || signal::dispatch(emu)
    {
        return;
    }

    misc::dispatch(emu);
}
