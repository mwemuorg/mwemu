use crate::emu;
use crate::winapi::helper;
pub fn SetFilePointer(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let distance = emu.regs().rdx as i32;
    let p_distance_high = emu.regs().r8;
    let move_method = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!SetFilePointer handle:0x{:x} dist:{} high:0x{:x} method:{}",
        h_file,
        distance,
        p_distance_high,
        move_method
    );
    if helper::handler_exist(h_file) {
        let pos = helper::handler_seek(h_file, distance as i64, move_method);
        emu.set_last_error(0);
        emu.regs_mut().rax = pos.max(0) as u64;
    } else {
        emu.regs_mut().rax = 0xFFFFFFFF;
    }
}
