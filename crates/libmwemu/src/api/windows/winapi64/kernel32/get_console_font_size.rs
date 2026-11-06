use crate::emu;

pub fn GetConsoleFontSize(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let font_index = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!GetConsoleFontSize h:0x{:x} font:{} => 8x16",
        h_console,
        font_index
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (16u64 << 16) | 8;
}
