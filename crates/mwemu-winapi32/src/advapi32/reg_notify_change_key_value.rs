use crate::emu;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let watch_subtree = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) != 0;
    let filter = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let event = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let async_ = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) != 0;
    log_red!(
        emu,
        "advapi32!RegNotifyChangeKeyValue hkey:0x{:x} subtree:{} filter:0x{:x}",
        hkey,
        watch_subtree,
        filter
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
