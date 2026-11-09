use crate::emu;

pub fn SafeArraySetRecordInfo(emu: &mut emu::Emu) {
    let sa = emu.regs().rcx;
    log_red!(emu, "oleaut32!SafeArraySetRecordInfo 0x{:x}", sa);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
