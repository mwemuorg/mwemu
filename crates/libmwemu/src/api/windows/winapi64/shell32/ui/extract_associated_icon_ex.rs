use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn ExtractAssociatedIconExA(emu: &mut emu::Emu) {
    extract_associated_icon_ex(emu, "ExtractAssociatedIconExA", false);
}

pub fn ExtractAssociatedIconExW(emu: &mut emu::Emu) {
    extract_associated_icon_ex(emu, "ExtractAssociatedIconExW", true);
}

fn extract_associated_icon_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("extractassociatediconexw");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("extractassociatediconexa");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    }
}
