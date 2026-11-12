use crate::emu;
use crate::winapi::common::strings;

pub fn AppendMenuA(emu: &mut emu::Emu) {
    append_menu(emu, "AppendMenuA", false);
}

fn append_menu(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hmenu = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let id_new_item = emu.regs().r8;
    let new_item_ptr = emu.regs().r9;
    let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
        strings::read_text(emu, new_item_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!{api} menu:0x{:x} flags:0x{:x} id:{} '{}'",
        hmenu,
        flags,
        id_new_item,
        text
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn AppendMenuW(emu: &mut emu::Emu) {
    append_menu(emu, "AppendMenuW", true);
}
