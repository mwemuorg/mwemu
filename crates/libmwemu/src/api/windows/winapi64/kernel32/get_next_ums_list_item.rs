use crate::emu;

pub fn GetNextUmsListItem(emu: &mut emu::Emu) {
    let ums_context = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetNextUmsListItem ctx:0x{:x} => 0 (end of list)",
        ums_context
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
