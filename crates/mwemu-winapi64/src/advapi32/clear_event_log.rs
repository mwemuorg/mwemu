use crate::emu;

pub fn ClearEventLogA(emu: &mut emu::Emu) {
    let event_log = emu.regs().rcx;
    let backup_ptr = emu.regs().rdx;
    let backup = if backup_ptr != 0 && backup_ptr > 0xffff {
        emu.maps.read_string(backup_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!ClearEventLogA handle:0x{:x} backup:'{}'",
        event_log,
        backup
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ClearEventLogW(emu: &mut emu::Emu) {
    let event_log = emu.regs().rcx;
    let backup_ptr = emu.regs().rdx;
    let backup = if backup_ptr != 0 && backup_ptr > 0xffff {
        emu.maps.read_wide_string(backup_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!ClearEventLogW handle:0x{:x} backup:'{}'",
        event_log,
        backup
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
