use crate::emu;

pub fn CreateEventA(emu: &mut emu::Emu) {
    let ev_attr_ptr = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!CreateEventA error reading param") as u64;
    let bManualReset = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!CreateEventA error reading param");
    let bInitialState = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!CreateEventA error reading param");
    let name_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernel32!CreateEventA error reading param") as u64;

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!CreateEventA `{}`", name);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
