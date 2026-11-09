use crate::emu;

pub fn WaitForMultipleObjectsEx(emu: &mut emu::Emu) {
    let count = emu.regs().rcx;
    let handles = emu.regs().rdx;
    let wait_all = emu.regs().r8;
    let ms = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!WaitForMultipleObjectsEx count:{} wait_all:{} ms:{}",
        count,
        wait_all,
        ms
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // WAIT_OBJECT_0
}
