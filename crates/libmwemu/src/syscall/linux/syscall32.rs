use crate::emu;

mod fs;
mod memory;
mod misc;
mod net;
mod proc;
mod signal;

fn trace_syscall32(emu: &mut emu::Emu, name: &str) {
    log::trace!(
        "{}** {} syscall {} {}",
        emu.colors.light_red,
        emu.pos,
        name,
        emu.colors.nc
    );
}

pub fn gateway(emu: &mut emu::Emu) {
    emu.regs_mut().sanitize32();

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
