use crate::emu;

pub fn CombineRgn(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let src1 = emu.regs().rdx;
    let src2 = emu.regs().r8;
    let mode = emu.regs().r9 as u32;
    let mode_name = match mode {
        1 => "RGN_AND",
        2 => "RGN_OR",
        3 => "RGN_XOR",
        4 => "RGN_DIFF",
        5 => "RGN_COPY",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CombineRgn dest:0x{:x} src1:0x{:x} src2:0x{:x} mode:{} ({})",
        dest,
        src1,
        src2,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}
