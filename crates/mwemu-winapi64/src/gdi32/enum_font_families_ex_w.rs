use crate::emu;

pub fn EnumFontFamiliesExW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let logfont = emu.regs().rdx;
    let proc_ptr = emu.regs().r8;
    let lparam = emu.regs().r9;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!EnumFontFamiliesExW hdc:0x{:x} lf:0x{:x} proc:0x{:x} lparam:0x{:x}",
        hdc,
        logfont,
        proc_ptr,
        lparam
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
