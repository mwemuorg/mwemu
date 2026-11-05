use crate::emu;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    let tid = emu.current_thread().id;
    log_red!(emu, "kernel32!GetCurrentThreadId = 0x{:x}", tid);
    emu.set_last_error(0);
    emu.regs_mut().rax = tid;
}
