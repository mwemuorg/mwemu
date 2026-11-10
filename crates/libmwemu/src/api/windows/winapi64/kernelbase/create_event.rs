use crate::emu;
use crate::winapi::helper;

pub fn CreateEventA(emu: &mut emu::Emu) {
    let security = emu.regs().rcx;
    let manual_reset = emu.regs().rdx;
    let initial_state = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("event_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateEventA '{}' manual:{} initial:{} => 0x{:x}",
        name,
        manual_reset,
        initial_state,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    let security = emu.regs().rcx;
    let manual_reset = emu.regs().rdx;
    let initial_state = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("event_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateEventW '{}' manual:{} initial:{} => 0x{:x}",
        name,
        manual_reset,
        initial_state,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
