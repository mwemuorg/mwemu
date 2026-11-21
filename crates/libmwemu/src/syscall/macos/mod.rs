use crate::emu::Emu;
use crate::threading::process::exit_status;

pub(crate) mod bsd;
pub mod syscall_aarch64;
pub mod syscall_x86_64;

fn set_fork_result(emu: &mut Emu, pid: u64, child: bool) {
    if emu.cfg.arch.is_aarch64() {
        let regs = emu.regs_aarch64_mut();
        regs.x[0] = pid;
        regs.x[1] = u64::from(child);
        regs.nzcv.c = false;
    } else {
        emu.regs_mut().rax = pid;
        emu.regs_mut().rdx = u64::from(child);
        emu.flags_mut().f_cf = false;
    }
}

fn fork(emu: &mut Emu) {
    let child = emu.fork_begin();
    log::trace!("Darwin fork: child {child}");
    set_fork_result(emu, child, true);
}

fn exit(emu: &mut Emu, status: u64) {
    log::trace!("Darwin exit({status})");
    if let Some(child) = emu.process_exit(exit_status(status)) {
        set_fork_result(emu, child, false);
    }
}

fn dispatch(emu: &mut Emu, number: u64, args: [u64; 6]) {
    const SYS_EXIT: u64 = 1;
    const SYS_FORK: u64 = 2;
    match number {
        SYS_EXIT => exit(emu, args[0]),
        SYS_FORK => fork(emu),
        _ => {
            let result = match bsd::Operation::from_number(number) {
                Some(operation) => operation.run(emu, args),
                None => {
                    log::warn!("unsupported Darwin syscall {number:#x}");
                    Err(bsd::ENOSYS)
                }
            };
            let failed = result.is_err();
            let value = result.unwrap_or_else(u64::from);
            if emu.cfg.arch.is_aarch64() {
                let regs = emu.regs_aarch64_mut();
                regs.x[0] = value;
                regs.nzcv.c = failed;
            } else {
                emu.regs_mut().rax = value;
                emu.flags_mut().f_cf = failed;
            }
        }
    }
}
