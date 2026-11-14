use crate::emu;

pub fn GetDIBColorTable(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let start = emu.regs().rdx;
    let entries = emu.regs().r8;
    let colors = emu.regs().r9;
    if colors != 0 && entries > 0 {
        let n = entries.min(256) as usize;
        for i in 0..n {
            let offset = colors + (i as u64) * 4;
            let gray = (i * 255 / n.max(1)) as u32;
            let rgbq = gray | (gray << 8) | (gray << 16);
            emu.maps.write_dword(offset, rgbq);
        }
    }
    log_red!(
        emu,
        "gdi32!GetDIBColorTable hdc:0x{:x} start:{} entries:{} colors:0x{:x}",
        hdc,
        start,
        entries,
        colors
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = entries.min(256);
}
