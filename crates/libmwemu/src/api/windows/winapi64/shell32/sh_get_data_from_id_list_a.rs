use crate::emu;

pub fn SHGetDataFromIDListA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let s = emu.maps.read_string(str_ptr);
    log_red!(emu, "shell32!SHGetDataFromIDListA '{}'  0x{:x}", s, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
