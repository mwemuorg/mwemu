use crate::emu;

pub fn AdjustCalendarDate(emu: &mut emu::Emu) {
    let _CALDATETIME = emu.regs().rcx as u32;
    let _int = emu.regs().rdx as u32;
    let _int = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!AdjustCalendarDate");
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
