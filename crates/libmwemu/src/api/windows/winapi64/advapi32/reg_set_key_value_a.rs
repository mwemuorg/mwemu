use crate::emu;

pub fn RegSetKeyValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "advapi32!RegSetKeyValueA hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
