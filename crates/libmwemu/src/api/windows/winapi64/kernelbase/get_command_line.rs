use crate::emu;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernelbase!GetCommandLineA '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernelbase!GetCommandLineW '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
