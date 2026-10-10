use crate::emu;

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    let msg = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!OutputDebugStringW `{}`", msg);

    emu.regs_mut().rax = 0;
}
