use crate::emu;

pub fn RegSetValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "advapi32!RegSetValueA hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegSetValueW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(
        emu,
        "advapi32!RegSetValueW hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
