use crate::emu;
use crate::winapi::helper;

pub fn CreateEventA(emu: &mut emu::Emu) {
    let ev_attr_ptr = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0) as u64;
    let bManualReset = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let bInitialState = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0) as u64;

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
