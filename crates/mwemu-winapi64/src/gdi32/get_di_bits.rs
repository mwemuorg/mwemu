use crate::emu;

pub fn GetDIBits(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hbmp = emu.regs().rdx;
    let start = emu.regs().r8;
    let lines = emu.regs().r9;
    let lpvbits = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let lpbmi = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if lpbmi != 0 {
        emu.maps.write_dword(lpbmi, 40); // biSize = BITMAPINFOHEADER
        emu.maps.write_dword(lpbmi + 4, 1920); // biWidth
        emu.maps.write_dword(lpbmi + 8, 1080u32 as i32 as u32); // biHeight
        emu.maps.write_word(lpbmi + 12, 1); // biPlanes
        emu.maps.write_word(lpbmi + 14, 32); // biBitCount
    }
    log_red!(
        emu,
        "gdi32!GetDIBits hdc:0x{:x} hbmp:0x{:x} start:{} lines:{} bits:0x{:x} bmi:0x{:x}",
        hdc,
        hbmp,
        start,
        lines,
        lpvbits,
        lpbmi
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = lines;
}
