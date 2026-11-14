use crate::emu;

pub fn GetCharWidth32A(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32A");
}

fn get_char_width32(emu: &mut emu::Emu, api: &str) {
    let hdc = emu.regs().rcx;
    let first = emu.regs().rdx;
    let last = emu.regs().r8;
    let buf = emu.regs().r9;
    let count = last.saturating_sub(first).min(255) + 1;
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + i * 4, 8);
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

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32W");
}
