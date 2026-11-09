use crate::emu;
use crate::winapi::helper;

pub fn fopen(emu: &mut emu::Emu) {
    let filepath_ptr = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    if filepath_ptr == 0 || mode_ptr == 0 {
        emu.regs_mut().rax = 0xffffffff;
        log_red!(emu, "msvcrt!fopen null ptrs");
    } else {
        let filepath = emu.maps.read_string(filepath_ptr);
        let mode = emu.maps.read_string(mode_ptr);
        log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);
        emu.regs_mut().rax = helper::handler_create(&filepath);
    }
}
