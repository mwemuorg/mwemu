use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DragQueryPoint(emu: &mut emu::Emu) {
    let hdrop = WinApiAbi::arg(emu, 0);
    let point_ptr = WinApiAbi::arg(emu, 1);
    log_red!(emu, "shell32!DragQueryPoint hdrop:0x{:x}", hdrop);
    if point_ptr != 0 {
        emu.maps.write_dword(point_ptr, 0);
        emu.maps.write_dword(point_ptr + 4, 0);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
