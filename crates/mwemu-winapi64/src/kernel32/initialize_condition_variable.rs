use crate::emu;

pub fn InitializeConditionVariable(emu: &mut emu::Emu) {
    let cv = emu.regs().rcx;
    log_red!(emu, "kernel32!InitializeConditionVariable 0x{:x}", cv);
    if cv != 0 {
        emu.maps.write_qword(cv, 0);
    }
}
