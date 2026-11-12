use crate::emu;

pub fn CoMarshalHresult(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "ole32!CoMarshalHresult stream:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
