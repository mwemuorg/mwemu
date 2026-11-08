use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CreateEventW(emu: &mut emu::Emu) {
    let attributes = arg(emu, 0) as u64;
    let manual_reset = arg(emu, 1) as u64;
    let initial_state = arg(emu, 2) as u64;
    let name_ptr = arg(emu, 3) as u64;

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
    stdcall_ret(emu, 4);
}
