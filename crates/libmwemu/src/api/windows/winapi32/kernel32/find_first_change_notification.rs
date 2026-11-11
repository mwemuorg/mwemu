use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstChangeNotificationA");
    emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationA");
}

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstChangeNotificationW");
    emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationW");
}
