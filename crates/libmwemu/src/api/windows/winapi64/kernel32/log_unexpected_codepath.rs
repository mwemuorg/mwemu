use crate::emu;

pub fn LogUnexpectedCodepath(emu: &mut emu::Emu) {
    let msg_ptr = emu.regs().rcx;
    let msg = emu.maps.read_string(msg_ptr);
    log_red!(emu, "kernel32!LogUnexpectedCodepath '{}'", msg);
}
