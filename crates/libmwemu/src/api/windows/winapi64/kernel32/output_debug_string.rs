use crate::emu;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    let msg = emu.maps.read_string(str_ptr);

    log_red!(emu, "kernel32!OutputDebugStringA `{}`", msg);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    let msg = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!OutputDebugStringW `{}`", msg);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
