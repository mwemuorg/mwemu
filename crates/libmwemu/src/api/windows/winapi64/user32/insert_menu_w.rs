use crate::emu;

pub fn InsertMenuW(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    let position = emu.regs().rdx as u32;
    let flags = emu.regs().r8 as u32;
    let id_new_item = emu.regs().r9;
    let new_item_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
        emu.maps.read_wide_string(new_item_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!InsertMenuW menu:0x{:x} pos:{} flags:0x{:x} '{}'",
        hmenu,
        position,
        flags,
        text
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
