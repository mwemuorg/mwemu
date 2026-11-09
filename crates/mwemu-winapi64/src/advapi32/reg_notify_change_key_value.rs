use crate::emu;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let watch_subtree = emu.regs().rdx != 0;
    let filter = emu.regs().r8;
    let event = emu.regs().r9;
    let _async = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) != 0;
    log_red!(
        emu,
        "advapi32!RegNotifyChangeKeyValue hkey:0x{:x} subtree:{} filter:0x{:x}",
        hkey,
        watch_subtree,
        filter
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
