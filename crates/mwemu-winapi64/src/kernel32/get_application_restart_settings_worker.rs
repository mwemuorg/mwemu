use crate::emu;

pub fn GetApplicationRestartSettingsWorker(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetApplicationRestartSettingsWorker h:0x{:x}",
        h_process
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80004005u64;
}
