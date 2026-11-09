use crate::emu;

pub fn DragQueryPoint(emu: &mut emu::Emu) {
    let hdrop = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let point_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
