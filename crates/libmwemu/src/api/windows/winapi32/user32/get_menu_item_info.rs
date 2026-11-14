use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetMenuItemInfoA(emu: &mut emu::Emu) {
    get_menu_item_info(emu, "GetMenuItemInfoA");
}

fn get_menu_item_info(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn GetMenuItemInfoW(emu: &mut emu::Emu) {
    get_menu_item_info(emu, "GetMenuItemInfoW");
}
