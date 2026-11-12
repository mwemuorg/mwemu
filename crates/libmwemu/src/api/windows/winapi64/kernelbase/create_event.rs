use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateEventA(emu: &mut emu::Emu) {
    create_event(emu, "CreateEventA", false);
}

fn create_event(emu: &mut emu::Emu, api: &str, wide: bool) {
    let security = emu.regs().rcx;
    let manual_reset = emu.regs().rdx;
    let initial_state = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("event_{}", name));
    log_red!(
        emu,
        "kernelbase!{api} '{}' manual:{} initial:{} => 0x{:x}",
        name,
        manual_reset,
        initial_state,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    create_event(emu, "CreateEventW", true);
}
