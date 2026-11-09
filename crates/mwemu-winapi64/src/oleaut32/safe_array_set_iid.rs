use crate::emu;

pub fn SafeArraySetIID(emu: &mut emu::Emu) {
    let sa = emu.regs().rcx;
    log_red!(emu, "oleaut32!SafeArraySetIID 0x{:x}", sa);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
