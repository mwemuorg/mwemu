use crate::emu;

pub fn WaitForSingleObjectEx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ms = emu.regs().rdx;
    let alertable = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!WaitForSingleObjectEx h:0x{:x} ms:{} alert:{}",
        handle,
        ms,
        alertable
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
}
