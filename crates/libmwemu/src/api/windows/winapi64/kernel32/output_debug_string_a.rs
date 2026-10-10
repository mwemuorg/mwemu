use crate::emu;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    let msg = emu.maps.read_string(str_ptr);

    log_red!(emu, "kernel32!OutputDebugStringA `{}`", msg);

    emu.regs_mut().rax = 0;
}
