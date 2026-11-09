use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreW(emu: &mut emu::Emu) {
    let _attrs = emu.regs().rcx;
    let _initial_count = emu.regs().rdx as u32;
    let _max_count = emu.regs().r8 as u32;
    let name_ptr = emu.regs().r9;

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "kernel32!CreateSemaphoreW '{}'", name);
    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreW");
}
