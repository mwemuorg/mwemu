use crate::emu;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernelbase!FindNextFileA '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernelbase!FindNextFileW '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
