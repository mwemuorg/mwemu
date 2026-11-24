use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::files;

mod fs;
mod memory;
mod misc;
mod net;
mod poll;
mod proc;
mod signal;
mod thread;
mod time;

pub(super) use thread::vfork_parent;
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
    if emu.resume_due_linux_process() {
        return;
    }
    if !emu.call_syscall_hook(nr) {
        return;
    }
    if files::dispatch(emu, Arch::X86_64, nr) {
        return;
    }
    if thread::dispatch(emu)
        || poll::dispatch(emu)
        || time::dispatch(emu)
        || fs::dispatch(emu)
        || proc::dispatch(emu)
        || net::dispatch(emu)
        || memory::dispatch(emu)
        || signal::dispatch(emu)
    {
        return;
    }

    misc::dispatch(emu);
}
