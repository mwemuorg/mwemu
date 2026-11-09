use crate::emu;

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let count = emu.regs().rdx;
    let buf = emu.regs().r8;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_wide_string(buf, face);
    }
    log_red!(
        emu,
        "gdi32!GetTextFaceW hdc:0x{:x} count:{} => '{}'",
        hdc,
        count,
        face
    );
    emu.regs_mut().rax = face.len() as u64;
}
