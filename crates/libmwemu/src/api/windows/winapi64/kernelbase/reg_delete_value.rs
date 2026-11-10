use crate::emu;

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "kernelbase!RegDeleteValueA hkey:0x{:x} '{}'",
        hkey,
        name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteValueW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(
        emu,
        "kernelbase!RegDeleteValueW hkey:0x{:x} '{}'",
        hkey,
        name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
