use crate::emu;

pub(super) fn _set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs().rcx;
    log_red!(emu, "wincrt!_set_app_type app_type: 0x{:x}", app_type);
    emu.regs_mut().rax = 0;
}
