use crate::emu;

pub fn CoFileTimeNow(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!CoFileTimeNow arg1:0x{:x}", arg1);
    if emu.regs().rcx != 0 {
        emu.maps.write_qword(emu.regs().rcx, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
