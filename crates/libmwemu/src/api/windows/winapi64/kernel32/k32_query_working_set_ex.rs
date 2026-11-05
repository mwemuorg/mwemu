use crate::emu;

pub fn K32QueryWorkingSetEx(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let p_info = emu.regs().rdx;
    let cb = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!K32QueryWorkingSetEx hProc:0x{:x} pInfo:0x{:x} cb:{}",
        h_process,
        p_info,
        cb
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
