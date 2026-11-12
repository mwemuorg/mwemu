use crate::emu;

pub fn GetCharABCWidthsA(emu: &mut emu::Emu) {
    get_char_abc_widths(emu, "GetCharABCWidthsA");
}

fn get_char_abc_widths(emu: &mut emu::Emu, api: &str) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let abc = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if abc != 0 {
        for i in 0..count {
            let off = abc + i * 12;
            emu.maps.write_dword(off, 1); // abcA
            emu.maps.write_dword(off + 4, 6); // abcB
            emu.maps.write_dword(off + 8, 1); // abcC
        }
    }
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    emu.regs_mut().rax = 1;
}

pub fn GetCharABCWidthsW(emu: &mut emu::Emu) {
    get_char_abc_widths(emu, "GetCharABCWidthsW");
}
