use crate::emu;
use crate::winapi::helper;

pub fn CertCompareIntegerBlob(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "crypt32!CertCompareIntegerBlob 0x{:x} 0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
