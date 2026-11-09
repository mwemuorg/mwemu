use crate::emu;

pub fn SetParent(emu: &mut emu::Emu) {
    let hwnd_child = emu.regs().rcx;
    let hwnd_parent = emu.regs().rdx;
    log_red!(
        emu,
        "user32!SetParent child:0x{:x} parent:0x{:x}",
        hwnd_child,
        hwnd_parent
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = hwnd_parent;
}
