use crate::emu;

pub fn GetSystemPaletteEntries(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let start = emu.regs().rdx as u32;
    let entries = emu.regs().r8 as u32;
    let pe = emu.regs().r9;
    if pe != 0 && entries > 0 {
        for i in 0..entries.min(256) {
            let offset = pe + (i as u64) * 4;
            let gray = ((start + i) * 255 / 256) as u8;
            let entry = (gray as u32) | ((gray as u32) << 8) | ((gray as u32) << 16);
            emu.maps.write_dword(offset, entry);
        }
    }
    log_red!(
        emu,
        "gdi32!GetSystemPaletteEntries hdc:0x{:x} start:{} entries:{} pe:0x{:x}",
        hdc,
        start,
        entries,
        pe
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = entries.min(256) as u64;
}
