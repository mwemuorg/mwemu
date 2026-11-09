use crate::emu;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let watch_subtree = emu.regs().rdx;
    let notify_filter = emu.regs().r8;
    let h_event = emu.regs().r9;
    let async_notify = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!RegNotifyChangeKeyValue hkey:0x{:x} filter:0x{:x} async:{}",
        hkey,
        notify_filter,
        async_notify
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
