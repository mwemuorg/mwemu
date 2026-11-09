use crate::emu;

pub fn EtwEventEnabled(emu: &mut emu::Emu) {
    let reg_handle = emu.regs().rcx;
    let event_descriptor = emu.regs().rdx;
    let level = if event_descriptor != 0 {
        emu.maps.read_byte(event_descriptor).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "kernel32!EventEnabled handle:0x{:x} level:{}",
        reg_handle,
        level
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
