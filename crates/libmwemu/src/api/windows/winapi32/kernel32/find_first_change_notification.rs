use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationA");
}

fn find_first_change_notification(emu: &mut emu::Emu, api: &str) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationW");
}
