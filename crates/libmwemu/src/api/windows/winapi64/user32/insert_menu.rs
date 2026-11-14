use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn InsertMenuA(emu: &mut emu::Emu) {
    insert_menu(emu, "InsertMenuA", false);
}

fn insert_menu(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hmenu = emu.regs().rcx;
    let position = emu.regs().rdx as u32;
    let flags = emu.regs().r8 as u32;
    let id_new_item = emu.regs().r9;
    let new_item_ptr = WinApiAbi::arg(emu, 4);
    let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
        strings::read_text(emu, new_item_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!{api} menu:0x{:x} pos:{} flags:0x{:x} '{}'",
        hmenu,
        position,
        flags,
        text
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InsertMenuW(emu: &mut emu::Emu) {
    insert_menu(emu, "InsertMenuW", true);
}
