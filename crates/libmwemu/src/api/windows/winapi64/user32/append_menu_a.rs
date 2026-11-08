use crate::emu;

pub fn AppendMenuA(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let id_new_item = emu.regs().r8;
    let new_item_ptr = emu.regs().r9;
    let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
        emu.maps.read_string(new_item_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!AppendMenuA menu:0x{:x} flags:0x{:x} id:{} '{}'",
        hmenu,
        flags,
        id_new_item,
        text
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
