use crate::emu;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() as u32 {
        0 => super::trace_syscall32(emu, "restart_syscall"),
        1 => handle_syscall32_exit(emu),
        2 => super::trace_syscall32(emu, "fork"),
        7 => super::trace_syscall32(emu, "waitpid"),
        11 => handle_syscall32_execve(emu),
        13 => super::trace_syscall32(emu, "time"),
        _ => return false,
    }

    true
}

fn handle_syscall32_exit(emu: &mut emu::Emu) {
    super::trace_syscall32(emu, "exit");
    emu.stop();
}

fn handle_syscall32_execve(emu: &mut emu::Emu) {
    let cmd = emu.maps.read_string(emu.regs().rbx);
    log::trace!(
        "{}** {} syscall execve()  cmd: {} {}",
        emu.colors.light_red,
        emu.pos,
        cmd,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0;
}
