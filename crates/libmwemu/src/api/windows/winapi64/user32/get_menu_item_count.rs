use crate::emu;
use crate::winapi::helper;

pub fn GetMenuItemCount(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    log_red!(emu, "user32!GetMenuItemCount hmenu:0x{:x}", hmenu);
    let count = if helper::handler_exist(hmenu) {
        1u64
    } else {
        0xffffffffu64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = count;
}
