use crate::emu;
use crate::winapi::common::strings;

pub fn ClearEventLogA(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogA", false);
}

fn clear_event_log(emu: &mut emu::Emu, api: &str, wide: bool) {
    let event_log = emu.regs().rcx;
    let backup_ptr = emu.regs().rdx;
    let backup = if backup_ptr != 0 && backup_ptr > 0xffff {
        strings::read_text(emu, backup_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!{api} handle:0x{:x} backup:'{}'",
        event_log,
        backup
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ClearEventLogW(emu: &mut emu::Emu) {
    clear_event_log(emu, "ClearEventLogW", true);
}
