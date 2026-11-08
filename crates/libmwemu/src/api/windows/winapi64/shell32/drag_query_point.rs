use crate::emu;

pub fn DragQueryPoint(emu: &mut emu::Emu) {
    let hdrop = emu.regs().rcx;
    let point_ptr = emu.regs().rdx;
    log_red!(emu, "shell32!DragQueryPoint hdrop:0x{:x}", hdrop);
    if point_ptr != 0 {
        emu.maps.write_dword(point_ptr, 0);
        emu.maps.write_dword(point_ptr + 4, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
