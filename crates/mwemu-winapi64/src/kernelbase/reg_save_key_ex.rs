use crate::emu;

pub fn RegSaveKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let file_ptr = emu.regs().rdx;
    let _security = emu.regs().r8;
    let flags = emu.regs().r9;
    let filename = if file_ptr != 0 {
        emu.maps.read_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegSaveKeyExA hkey:0x{:x} '{}' flags:0x{:x}",
        hkey,
        filename,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}

pub fn RegSaveKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let file_ptr = emu.regs().rdx;
    let _security = emu.regs().r8;
    let flags = emu.regs().r9;
    let filename = if file_ptr != 0 {
        emu.maps.read_wide_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegSaveKeyExW hkey:0x{:x} '{}' flags:0x{:x}",
        hkey,
        filename,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
