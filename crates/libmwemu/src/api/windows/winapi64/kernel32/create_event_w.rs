use crate::emu;
use crate::winapi::helper;

pub fn CreateEventW(emu: &mut emu::Emu) {
    let attributes = emu.regs().rcx;
    let manual_reset = emu.regs().rdx;
    let initial_state = emu.regs().r8;
    let name_ptr = emu.regs().r9;

    let mut name = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_wide_string(name_ptr);
    }

    log_red!(
        emu,
        "kernel32!CreateEventW attr: 0x{:x} manual_reset: {} init_state: {} name: {}",
        attributes,
        manual_reset,
        initial_state,
        name
    );

    let uri = format!("event://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
