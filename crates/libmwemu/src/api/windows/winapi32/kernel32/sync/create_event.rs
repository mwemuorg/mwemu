use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateEventA(emu: &mut emu::Emu) {
    let ev_attr_ptr = WinApiAbi::arg(emu, 0);
    let bManualReset = WinApiAbi::arg(emu, 1) as u32;
    let bInitialState = WinApiAbi::arg(emu, 2) as u32;
    let name_ptr = WinApiAbi::arg(emu, 3);

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!CreateEventA `{}`", name);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateEventW");
    emu.regs_mut().rax = helper::handler_create("CreateEventW");
}
