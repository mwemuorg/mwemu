use crate::emu;

pub fn IsCharUpperA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernelbase!IsCharUpperA '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
